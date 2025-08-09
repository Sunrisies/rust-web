use crate::models::sluice_devices::{self};
use crate::mqtt::DeviceUpdate;
use rumqttc::{AsyncClient, QoS};

use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

// 订阅设备主题
pub async fn subscribe_device(client: &AsyncClient, device: &sluice_devices::Model) {
    let subscribe_topic = device.subscribe_topic.as_str();
    let publish_topic = device.publish_topic.as_str();

    if let Err(e) = client.subscribe(subscribe_topic, QoS::AtLeastOnce).await {
        log::error!("订阅主题 {} 失败: {}", subscribe_topic, e);
    } else {
        log::info!("成功订阅主题: {}", subscribe_topic);
    }

    if let Err(e) = client.subscribe(publish_topic, QoS::AtLeastOnce).await {
        log::error!("订阅主题 {} 失败: {}", publish_topic, e);
    } else {
        log::info!("成功订阅主题: {}", publish_topic);
    }
}

// 取消订阅设备主题
pub async fn unsubscribe_device(client: &AsyncClient, subscribe_topic: &str, publish_topic: &str) {
    if let Err(e) = client.unsubscribe(subscribe_topic).await {
        log::error!("取消订阅主题 {} 失败: {}", subscribe_topic, e);
    }

    if let Err(e) = client.unsubscribe(publish_topic).await {
        log::error!("取消订阅主题 {} 失败: {}", publish_topic, e);
    }
}

// 处理设备更新
pub async fn handle_device_update(
    client: &Arc<Mutex<AsyncClient>>,
    subscribed_devices: &Arc<RwLock<HashSet<String>>>,
    update: DeviceUpdate,
) {
    let client_lock = client.lock().await;
    let mut subscribed_set = subscribed_devices.write().await;

    match update {
        DeviceUpdate::Added(device) => {
            if !subscribed_set.contains(&device.subscribe_topic) {
                subscribe_device(&client_lock, &device).await;
                subscribed_set.insert(device.subscribe_topic.clone());
                log::info!("已添加并订阅新设备: {}", device.device_name);
            }
        }
        DeviceUpdate::Removed(device) => {
            // log::info!("收到设备移除通知: {}", subscribe_topic);
            log::info!("取消订阅设备: {:?}", subscribed_set);
            if subscribed_set.contains(&device.subscribe_topic) {
                // 获取设备完整信息以获取publish_topic
                unsubscribe_device(&client_lock, &device.subscribe_topic, &device.publish_topic)
                    .await;
                subscribed_set.remove(&device.subscribe_topic);
                log::info!("已移除并取消订阅设备: {}", device.device_name);
                log::info!("取消订阅设备: {:?}", subscribed_set);
            }
        }
        DeviceUpdate::Updated(device) => {
            // 先取消旧的订阅（如果存在）
            if subscribed_set.contains(&device.subscribe_topic) {
                unsubscribe_device(&client_lock, &device.subscribe_topic, &device.publish_topic)
                    .await;
            }

            // 重新订阅
            subscribe_device(&client_lock, &device).await;
            subscribed_set.insert(device.subscribe_topic.clone());
            log::info!("已更新设备订阅: {}", device.device_name);
        }
    }
}
