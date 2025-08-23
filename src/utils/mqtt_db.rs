use crate::models::sluice_command::{self, Entity as SluiceCommand};
use crate::models::sluice_data::{self, Entity as SluiceData};
use chrono::Utc;
use num_traits::cast::FromPrimitive;
use sea_orm::DatabaseConnection;
use sea_orm::{prelude::Decimal, ActiveValue, EntityTrait, Set};
use serde_json::Value;
use std::collections::HashMap;

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
    let device_info = params.get("DEVICE_INFO").map(|s| s.to_string());
    log::info!("device_id:{:?}", device_id);
    let command = sluice_command::ActiveModel {
        user_id: ActiveValue::Set(user_id),
        device_id: ActiveValue::Set(device_id),
        topic: ActiveValue::Set(topic.to_string()),
        create_time: Set(Utc::now().timestamp()),
        content: Set(payload.to_string()),
        device_info: Set(device_info),
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
