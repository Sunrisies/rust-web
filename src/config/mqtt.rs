use crate::models::sluice_command::{self, Column, Entity as SluiceCommand};
use crate::models::sluice_devices::{self, Entity as SluiceDevicesEntity};
use crate::models::user::{self, Entity as UserEntity};
use crate::mqtt_db::{insert_sluice_command, insert_sluice_data};
use crate::sluice_mqtt::{handle_device_update, subscribe_device};
use actix_web::web;
use chrono::{Duration as ChronoDuration, TimeZone, Utc};
use num_traits::cast::FromPrimitive;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish, QoS};
use sea_orm::{
    prelude::Decimal, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::{mpsc, oneshot, Mutex, RwLock};
use tokio::task;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub enum MqttError {
    ConnectionError(String),
    PublishError(String),
    TimeoutError,
    ReceiveError(String),
}

impl std::fmt::Display for MqttError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MqttError::ConnectionError(ref err) => write!(f, "Connection error: {}", err),
            MqttError::PublishError(ref err) => write!(f, "Publish error: {}", err),
            MqttError::TimeoutError => write!(f, "Timeout error"),
            MqttError::ReceiveError(ref err) => write!(f, "Receive error: {}", err),
        }
    }
}
// 新增：设备状态监控结构
#[derive(Debug, Clone)]
pub struct DeviceStatusMonitor {
    start_openhgt: Option<Decimal>,
    end_openhgt: Option<Decimal>,
    last_openhgt: Option<Decimal>,
    change_time: i64,
    d_topic: String,
    cancellation_token: Option<CancellationToken>, // 用于取消之前的任务
}
pub struct MqttClient {
    client: Arc<Mutex<AsyncClient>>,
    response_channels: Arc<Mutex<HashMap<String, oneshot::Sender<Publish>>>>,
    request_times: Arc<Mutex<HashMap<String, Instant>>>,
    // 新增：设备更新通道
    device_update_tx: mpsc::Sender<DeviceUpdate>,
}

async fn query_device_command(db_pool: &DatabaseConnection, params: &DeviceStatusMonitor) -> bool {
    let start_time = Utc
        .timestamp_opt(params.change_time, 0)
        .single()
        .expect("Invalid timestamp")
        .checked_sub_signed(ChronoDuration::seconds(10))
        .expect("Timestamp overflow")
        .timestamp();
    let end_time = Utc
        .timestamp_opt(params.change_time, 0)
        .single()
        .expect("Invalid timestamp")
        .checked_add_signed(ChronoDuration::seconds(10))
        .expect("Timestamp overflow")
        .timestamp();
    match SluiceCommand::find()
        .filter(sluice_command::Column::Topic.eq(&params.d_topic))
        .filter(Column::CreateTime.gte(start_time))
        .filter(Column::CreateTime.lte(end_time))
        .count(db_pool)
        .await
    {
        Ok(count) if count > 0 => {
            log::info!("查询到 {} 条命令记录", count);
            return false;
        }
        Ok(_) => {
            log::info!("未查询到命令记录");
            return true;
        }
        Err(e) => {
            log::error!("查询命令记录时发生错误: {}", e);
            return true;
        }
    };
}

// 定义设备更新消息
#[derive(Debug)]
pub enum DeviceUpdate {
    Added(sluice_devices::Model),
    Removed(sluice_devices::Model), // subscribe_topic
    Updated(sluice_devices::Model),
}
async fn handle_mqtt_message(
    p: &Publish,
    monitors_clone: Arc<Mutex<HashMap<String, DeviceStatusMonitor>>>,
    db_pool_clone: web::Data<DatabaseConnection>,
) {
    let payload = String::from_utf8_lossy(&p.payload);
    if let Ok(json) = serde_json::from_str::<Value>(&payload) {
        if let Some(openhgt) = json.get("OPENHGT") {
            let mut monitors = monitors_clone.lock().await;
            let cstmp = json
                .get("CSTAMP")
                .and_then(|v| v.as_i64())
                .unwrap_or_else(|| {
                    // 在这里可以执行一些逻辑来生成默认值
                    println!("CSTAMP 不存在，使用默认值");
                    0
                });
            let topic = p.topic.clone();
            let d_topic = format!("D{}", topic);
            if let Some(monitor) = monitors.get_mut(&d_topic) {
                let openhgt_str = Some(Decimal::from_f64(openhgt.as_f64().unwrap()).unwrap());
                // 检查状态是否变化
                if monitor.last_openhgt != openhgt_str {
                    if let Some(token) = monitor.cancellation_token.take() {
                        token.cancel();
                        log::info!("已取消之前的延时任务: {}", d_topic);
                    }
                    // 创建新的取消令牌
                    let new_token = CancellationToken::new();
                    monitor.cancellation_token = Some(new_token.clone());
                    monitor.last_openhgt = openhgt_str;
                    monitor.end_openhgt = openhgt_str;
                    monitor.change_time = cstmp;
                    let monitor_clone = monitor.clone();
                    let monitors_clone = monitors.clone();
                    tokio::spawn(async move {
                        tokio::select! {
                                                    _ = new_token.cancelled() => {
                                                            log::info!("任务被取消: {}", d_topic);
                                                        }
                                                        _ = tokio::time::sleep(Duration::from_secs(10)) => {
                                                    log::info!("延时结束: {}", d_topic);
                                                            if let Some(monitor) = monitors_clone.get(&d_topic) {
                                                                        // 检查取消令牌是否仍然是我们的（没有新的任务）
                                                                    if let Some(token) = &monitor.cancellation_token {

                                                                                                if token.is_cancelled() {
                                                    log::info!("任务已过时，跳过执行: {}", d_topic);
                                                    return;
                                                                         }
                                                 if query_device_command(&db_pool_clone, &monitor_clone).await {
                                                    log::info!("没有数据存储");
                                                    let (start, end) = match (monitor.start_openhgt, monitor.end_openhgt) {
                            (Some(s), Some(e)) => (s, e),
                            _ => panic!("start_openhgt or end_openhgt is None"), // 或给默认值
                        };
                                                    let content = format!(
                                                        "{{\"CONTENT\":\"本地操作\",\"START_OPENHGT\":\"{:?}\",\"END_OPENHGT\":\"{:?}\"}}",
                                                        start,
                                                        end
                                                    );
                                                    log::info!("发送命令: {}", content);
                                                    let mut params: HashMap<String, String> = HashMap::new();
                                                    params.insert("topic".to_string(), d_topic.to_string());
                                                    params.insert("payload".to_string(), content.to_string());
                                                    params.insert("USER_ID".to_string(), "11".to_string());

                                                    params.insert("DEVICE_ID".to_string(), "".to_string());
                                                    params.insert("DEVICE_INFO".to_string(), "".to_string());

                                                    insert_sluice_command(
                                                        &db_pool_clone, &params, &d_topic, &content, 1,
                                                    )
                                                    .await;
                                                    } else {
                                                        log::info!("有数据存储");
                                                    }
                                                }
                                                }
                                                       }
                                                }
                    });
                }
            } else {
                let monitor = DeviceStatusMonitor {
                    last_openhgt: Some(Decimal::from_f64(openhgt.as_f64().unwrap()).unwrap()),
                    change_time: cstmp,
                    d_topic: d_topic.clone(),
                    cancellation_token: None,
                    start_openhgt: Some(Decimal::from_f64(openhgt.as_f64().unwrap()).unwrap()),
                    end_openhgt: None,
                };
                monitors.insert(d_topic, monitor);
            }
        }
    }
}

impl MqttClient {
    pub async fn new(
        db_pool: web::Data<DatabaseConnection>,
        broker: &str,
        port: u16,
        client_id: &str,
    ) -> Self {
        let mqttoptions = MqttOptions::new(client_id, broker, port);
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
        let client = Arc::new(Mutex::new(client));
        let response_channels = Arc::new(Mutex::new(std::collections::HashMap::<
            String,
            oneshot::Sender<Publish>,
        >::new()));
        let response_channels_clone = Arc::clone(&response_channels);
        let response_listeners = Arc::new(Mutex::new(HashMap::new()));
        let request_times = Arc::new(Mutex::new(HashMap::<String, std::time::Instant>::new()));
        let initial_devices = SluiceDevicesEntity::find()
            .order_by_desc(sluice_devices::Column::Id)
            .all(db_pool.as_ref())
            .await
            .unwrap();

        let (device_update_tx, mut device_update_rx) = mpsc::channel(100);

        // 新增：当前已订阅设备集合
        let subscribed_devices = Arc::new(RwLock::new(HashSet::new()));

        {
            let client_lock = client.lock().await;
            let mut subscribed_set = subscribed_devices.write().await;

            for device in &initial_devices {
                subscribe_device(&client_lock, device).await;
                subscribed_set.insert(device.subscribe_topic.clone());
            }
        }
        let subscribed_devices_clone = Arc::clone(&subscribed_devices);
        let mut params: HashMap<String, String> = HashMap::new();
        // 新增：初始化设备状态监控和D主题接收时间记录
        let device_status_monitors: Arc<Mutex<HashMap<String, DeviceStatusMonitor>>> =
            Arc::new(Mutex::new(HashMap::new()));

        // 新增：启动定时器检查D主题消息接收状态
        let device_status_monitors_clone = Arc::clone(&device_status_monitors);

        // 后台任务处理 MQTT 消息
        task::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(Event::Incoming(Packet::Publish(p))) => {
                        let payload = String::from_utf8_lossy(&p.payload);
                        let topic = p.topic.clone();
                        log::info!("收到 MQTT 消息: {},{:?}", payload, p.topic);
                        // 检查是否是等待的响应
                        {
                            let mut channels = response_channels_clone.lock().await;
                            if let Some(tx) = channels.remove(&p.topic) {
                                if tx.send(p.clone()).is_ok() {
                                    log::info!("已发送响应到通道: {}", p.topic);
                                    continue; // 跳过数据库存储
                                }
                            }
                        }
                        let p_clone = p.clone();
                        let monitors_clone = device_status_monitors_clone.clone();
                        let db_pool_clone = db_pool.clone();
                        tokio::spawn(async move {
                            handle_mqtt_message(&p_clone, monitors_clone, db_pool_clone).await;
                        });
                        {
                            let mut listeners = response_listeners.lock().await;

                            // 如果是 D 开头的命令消息
                            if topic.starts_with("D") {
                                let response_topic = &topic[1..];

                                if let Ok(json) = serde_json::from_str::<Value>(&payload) {
                                    if let Some(user) = json.get("USER").and_then(|v| v.as_str()) {
                                        match UserEntity::find()
                                            .filter(user::Column::UserName.eq(user))
                                            .one(db_pool.as_ref())
                                            .await
                                        {
                                            Ok(Some(user)) => {
                                                params.insert(
                                                    "USER_ID".to_string(),
                                                    user.id.to_string(),
                                                );
                                                if let Some(device_id) =
                                                    json.get("DEVICE_ID").and_then(|v| v.as_str())
                                                {
                                                    params.insert(
                                                        "DEVICE_ID".to_string(),
                                                        device_id.to_string(),
                                                    );
                                                }
                                                // DEVICE_INFO
                                                if let Some(device_info) =
                                                    json.get("DEVICE_INFO").and_then(|v| v.as_str())
                                                {
                                                    params.insert(
                                                        "DEVICE_INFO".to_string(),
                                                        device_info.to_string(),
                                                    );
                                                }
                                                params
                                                    .insert("topic".to_string(), topic.to_string());
                                                params.insert(
                                                    "payload".to_string(),
                                                    payload.to_string(),
                                                );
                                            }
                                            Ok(None) => {}
                                            Err(_e) => {}
                                        }
                                    }
                                } else {
                                    log::error!("无效的 MQTT 负载: {}", payload);
                                }
                                // 记录监听开始时间
                                listeners.insert(response_topic.to_string(), Instant::now());
                            }
                            // 检查是否是等待的响应（被动监听）
                            else if let Some(start_time) = listeners.remove(&topic) {
                                let topic = params.get("topic").unwrap();
                                let payload = params.get("payload").unwrap();
                                let elapsed = start_time.elapsed();
                                if elapsed <= Duration::from_secs(5) {
                                    insert_sluice_command(&db_pool, &params, topic, payload, 1)
                                        .await;
                                } else {
                                    insert_sluice_command(
                                        db_pool.as_ref(),
                                        &params,
                                        topic,
                                        payload,
                                        0,
                                    )
                                    .await;
                                }
                                params.clear(); // 清空参数
                            }
                        }

                        // 先判断p.topic是否D开头，如果不是这个开头的就不用存储了
                        if p.topic.starts_with("D") {
                            continue;
                        } else {
                            // let mut monitors = device_status_monitors_clone.lock().await;
                            insert_sluice_data(db_pool.as_ref(), &p.topic, &payload).await;
                            // 新增：监控OPENSTA字段变化
                        }
                    }
                    Err(e) => {
                        log::error!("MQTT 错误: {}", e);
                    }
                    _ => {}
                }
            }
        });
        let client_clone = Arc::clone(&client);
        task::spawn(async move {
            while let Some(update) = device_update_rx.recv().await {
                handle_device_update(&client_clone, &subscribed_devices_clone, update).await;
            }
        });

        MqttClient {
            client,
            response_channels,
            request_times,
            device_update_tx,
        }
    }
    // 新增：公开方法用于通知设备更新
    pub async fn notify_device_update(&self, update: DeviceUpdate) {
        if let Err(e) = self.device_update_tx.send(update).await {
            log::error!("发送设备更新通知失败: {}", e);
        }
    }

    pub async fn publish_with_confirmation(
        &self,
        topic: &str,
        qos: QoS,
        payload: &str,
    ) -> Result<String, MqttError> {
        log::info!("MQTT 发布(带确认): {}, {}", topic, payload);

        // 确定响应主题（去掉开头的 'D'）
        let response_topic = if topic.starts_with("D") {
            &topic[1..]
        } else {
            topic
        };
        {
            let mut times = self.request_times.lock().await;
            times.insert(response_topic.to_string(), std::time::Instant::now());
        }

        log::info!("等待响应主题: {}", response_topic);

        // 创建响应通道
        let (tx, rx) = oneshot::channel();
        {
            let mut channels = self.response_channels.lock().await;
            channels.insert(response_topic.to_string(), tx);
        }

        // 发布消息
        let publish_result = {
            let client = self.client.lock().await;
            client.publish(topic, qos, false, payload).await
        };

        if let Err(e) = publish_result {
            log::error!("MQTT 发布失败: {}", e);

            // 清理通道
            let mut channels = self.response_channels.lock().await;
            channels.remove(response_topic);
            return Err(MqttError::PublishError(e.to_string()));
        }

        log::info!("MQTT 发布成功，等待响应: {}", response_topic);

        // 等待响应（最多5秒）
        let timeout = Duration::from_secs(5);
        match tokio::time::timeout(timeout, rx).await {
            Ok(Ok(_response)) => Ok("成功".to_string()),
            Ok(Err(_)) => {
                log::warn!("响应通道已关闭: {}", response_topic);
                Err(MqttError::ReceiveError("通道已关闭".to_string()))
            }
            Err(_) => {
                log::warn!("等待响应超时: {}", response_topic);
                // 清理通道
                let mut channels = self.response_channels.lock().await;
                channels.remove(response_topic);
                let mut times = self.request_times.lock().await;
                times.remove(response_topic);
                Err(MqttError::TimeoutError)
            }
        }
    }
}
