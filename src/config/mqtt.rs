use crate::models::sluice_command::{self, Entity as SluiceCommand};
use crate::models::sluice_data::{self, Entity as SluiceData};
use crate::models::sluice_devices::{self, Entity as SluiceDevicesEntity};
use crate::models::user::{self, Entity as UserEntity};
use crate::sluice_mqtt::{handle_device_update, subscribe_device};
use actix_web::web;
use chrono::Utc;
use num_traits::cast::FromPrimitive;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish, QoS};

use sea_orm::{
    prelude::Decimal, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder, Set,
};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::{mpsc, oneshot, Mutex, RwLock};

use tokio::task;

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

pub struct MqttClient {
    client: Arc<Mutex<AsyncClient>>,
    response_channels: Arc<Mutex<HashMap<String, oneshot::Sender<Publish>>>>,
    request_times: Arc<Mutex<HashMap<String, Instant>>>,

    // 新增：设备更新通道
    device_update_tx: mpsc::Sender<DeviceUpdate>,
    // 新增：当前已订阅设备集合
    subscribed_devices: Arc<RwLock<HashSet<String>>>,
    // response_listeners: Arc<Mutex<HashMap<String, Instant>>>,
}
// 插入闸门命令记录
pub async fn insert_sluice_command(
    db_pool: &DatabaseConnection,
    params: &HashMap<String, String>,
    topic: &str,
    payload: &str,
    push_status: i8,
) {
    let user_id = params
        .get("USER_ID")
        .and_then(|s| s.parse::<i64>().ok())
        .map(|id| id as i64)
        .expect("USER_ID 不能为空");

    let device_id = params.get("DEVICE_ID").map(|s| s.to_string());
    log::info!("device_id:{:?}", device_id);
    let command = sluice_command::ActiveModel {
        user_id: ActiveValue::Set(user_id),
        device_id: ActiveValue::Set(device_id),
        topic: ActiveValue::Set(topic.to_string()),
        create_time: Set(Utc::now().timestamp()),
        content: Set(payload.to_string()),
        push_status: Set(push_status),
        ..Default::default()
    };

    if let Err(e) = SluiceCommand::insert(command).exec(db_pool).await {
        log::error!("写入命令记录失败: {}", e);
    }
}
// 插入闸门数据记录
pub async fn insert_sluice_data(db_pool: &DatabaseConnection, topic: &str, payload: &str) {
    let json: Value = match serde_json::from_str(payload) {
        Ok(v) => v,
        Err(e) => {
            log::error!("解析JSON失败: {} - {}", payload, e);
            return;
        }
    };

    let sluice_data = sluice_data::ActiveModel {
        lvll: ActiveValue::Set(json["LVLL"].as_f64().map(|f| Decimal::from_f64(f).unwrap())),
        lvlh: ActiveValue::Set(json["LVLH"].as_f64().map(|f| Decimal::from_f64(f).unwrap())),
        ctrlmod: ActiveValue::Set(json["CTRLMOD"].as_i64().map(|i| i as i8)),
        opensta: ActiveValue::Set(json["OPENSTA"].as_i64().map(|i| i as i8)),
        openhgt: ActiveValue::Set(
            json["OPENHGT"]
                .as_f64()
                .map(|f| Decimal::from_f64(f).unwrap()),
        ),
        lvlb: ActiveValue::Set(json["LVLB"].as_f64().map(|f| Decimal::from_f64(f).unwrap())),
        lvla: ActiveValue::Set(json["LVLA"].as_f64().map(|f| Decimal::from_f64(f).unwrap())),
        iccid: ActiveValue::Set(json["ICCID"].as_str().map(|s| s.to_owned())),
        vbat: ActiveValue::Set(json["VBAT"].as_f64().map(|f| Decimal::from_f64(f).unwrap())),
        cstamp: ActiveValue::Set(json["CSTAMP"].as_i64()),
        csq: ActiveValue::Set(json["CSQ"].as_i64().map(|i| i as i32)),
        lo: ActiveValue::Set(json["LO"].as_f64().map(|f| Decimal::from_f64(f).unwrap())),
        la: ActiveValue::Set(json["LA"].as_f64().map(|f| Decimal::from_f64(f).unwrap())),
        sluice_id: ActiveValue::Set(topic.to_string()),
        ..Default::default()
    };

    if let Err(e) = SluiceData::insert(sluice_data).exec(db_pool).await {
        log::error!("写入数据记录失败: {}", e);
    }
}

// 定义设备更新消息
#[derive(Debug)]
pub enum DeviceUpdate {
    Added(sluice_devices::Model),
    Removed(sluice_devices::Model), // subscribe_topic
    Updated(sluice_devices::Model),
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
        // let listeners_clone = Arc::clone(&response_listeners);
        let request_times = Arc::new(Mutex::new(HashMap::<String, std::time::Instant>::new()));
        let initial_devices = SluiceDevicesEntity::find()
            .order_by_desc(sluice_devices::Column::Id)
            .all(db_pool.as_ref())
            .await
            .unwrap(); // 处理错误
                       // 订阅主题
                       // 新增：设备更新通道
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
                        {
                            let mut listeners = response_listeners.lock().await;

                            // 如果是 D 开头的命令消息
                            if topic.starts_with("D") {
                                let response_topic = &topic[1..];
                                log::info!(
                                    "收到命令消息，开始监听响应主题: {},{}",
                                    response_topic,
                                    payload
                                );
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

                                    log::info!(
                                        "✅ 成功收到响应: {} (耗时: {:?}),{},{:?}",
                                        topic,
                                        elapsed,
                                        payload,
                                        params.get("USER_ID")
                                    );
                                } else {
                                    insert_sluice_command(&db_pool, &params, topic, payload, 0)
                                        .await;
                                    log::warn!(
                                        "⚠️ 收到延迟响应: {} (耗时: {:?}, 超过5秒)",
                                        topic,
                                        elapsed
                                    );
                                }
                                params.clear(); // 清空参数
                            }
                        }

                        // 先判断p.topic是否D开头，如果不是这个开头的就不用存储了
                        if p.topic.starts_with("D") {
                            continue;
                        } else {
                            insert_sluice_data(db_pool.as_ref(), &p.topic, &payload).await;
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
            subscribed_devices,
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
