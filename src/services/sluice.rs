use crate::common::{PaginatedResponse, PaginationInfo, TimeQuery};
use crate::config::mqtt::MqttClient;
use crate::dto::sluice::SluiceRequest;
use crate::middleware::helpers::Resp;
use crate::middleware::helpers::SimpleResp;
use crate::models::sluice_command::Entity as SluiceCommandEntity;
use crate::models::sluice_data::{self, Entity as SluiceDataEntity};
use crate::models::user::{self, Entity as UserEntity};
use crate::utils::query_parameter::Query;
use crate::xlsx::generate_excel;
use crate::AppError;
use actix_web::http::header;
use actix_web::web::{self};
use actix_web::{Error, HttpResponse};
use chrono::Duration;
use chrono::Utc;
use log::*;
use rumqttc::QoS;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
pub async fn get_all_sluice(
    db: web::Data<DatabaseConnection>,
    query: Query<TimeQuery>,
) -> SimpleResp {
    // 获取时间戳范围
    let start_time = query
        .start_time
        .unwrap_or_else(|| (Utc::now() - Duration::hours(1)).timestamp());
    let end_time = query.end_time.unwrap_or_else(|| Utc::now().timestamp());
    let topic = query.topic.clone();
    // 查询数据
    let sluice = SluiceDataEntity::find()
        .filter(sluice_data::Column::SluiceId.eq(topic))
        .filter(sluice_data::Column::Cstamp.gte(start_time))
        .filter(sluice_data::Column::Cstamp.lte(end_time))
        .order_by_desc(sluice_data::Column::Id)
        .all(db.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(format!("数据库操作失败: {}", e)))?;
    let total = sluice.len() as u64;
    info!("total1: {}, ", total);
    // 获取分页用户数据
    let response = PaginatedResponse {
        data: sluice,
        pagination: PaginationInfo {
            total,
            total_pages: 1,      // 不分页，总页数为 1
            current_page: 1,     // 当前页为 1
            limit: total,        // 每页数量为总数量
            has_next: false,     // 没有下一页
            has_previous: false, // 没有上一页
        },
    };

    Resp::ok(response, "获取水位数据成功").to_json_result()
}

#[derive(Validate, Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GetSluiceQuery {
    pub topic: String,
}

pub async fn get_sluice(
    db: web::Data<DatabaseConnection>,
    query: Query<GetSluiceQuery>,
) -> SimpleResp {
    let sluice = SluiceDataEntity::find()
        .filter(sluice_data::Column::SluiceId.eq(query.topic.clone()))
        .order_by_desc(sluice_data::Column::Cstamp) // 按 CSTAMP 降序排序
        .limit(1) // 限制查询结果为 1 条
        .all(db.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(format!("数据库操作失败: {}", e)))?;
    // 如果没有找到数据，返回空数组
    let data = if sluice.is_empty() {
        vec![]
    } else {
        vec![sluice.first().unwrap().clone()]
    };

    // let data = deep_filter_data(sluice, vec![]);
    // 获取分页用户数据
    let response = PaginatedResponse {
        data: sluice,
        pagination: PaginationInfo {
            total: data.len() as u64,
            total_pages: 1,
            current_page: 1,
            limit: 1,
            has_next: false,
            has_previous: false,
        },
    };

    Resp::ok(response, "获取水位数据成功").to_json_result()
}

pub async fn get_all_sluice_control_log(
    db: web::Data<DatabaseConnection>,
    query: Query<TimeQuery>,
) -> SimpleResp {
    // 获取时间戳范围
    let start_time = query
        .start_time
        .unwrap_or_else(|| (Utc::now() - Duration::hours(1)).timestamp());
    let end_time = query.end_time.unwrap_or_else(|| Utc::now().timestamp());
    let topic = query.topic.clone();

    let (data, total) = match SluiceCommandEntity::find_with_user(
        db.as_ref(),
        start_time,
        end_time,
        &topic,
    )
    .await
    {
        Ok(sluice) => {
            let total = sluice.len() as u64;
            (sluice, total)
        }
        Err(e) => {
            error!("Error: {}", e);
            (vec![], 0)
        }
    };
    // 获取分页用户数据
    let response = PaginatedResponse {
        data,
        pagination: PaginationInfo {
            total,
            total_pages: 1,      // 不分页，总页数为 1
            current_page: 1,     // 当前页为 1
            limit: total,        // 每页数量为总数量
            has_next: false,     // 没有下一页
            has_previous: false, // 没有上一页
        },
    };

    Resp::ok(response, "获取水位数据成功").to_json_result()
}

pub async fn control_sluice(
    db: web::Data<DatabaseConnection>,
    control: web::Json<SluiceRequest>,
    mqtt_client: web::Data<MqttClient>,
) -> SimpleResp {
    let SluiceRequest {
        control_type,
        user_name,
        topic,
        device_id,
        device_info,
    } = control.into_inner();
    log::info!(
        "control_type: {}, user_name: {}, topic: {}, device_id: {}, device_info: {}",
        control_type,
        user_name,
        topic,
        device_id,
        device_info
    );
    // 去数据库查询用户是否存在
    match UserEntity::find()
        .filter(user::Column::UserName.eq(&user_name))
        .one(db.as_ref())
        .await
    {
        Ok(Some(user_record)) => {
            warn!("用户名 '{}' 已存在", user_name);
            info!("用户名 '{:?}' 不存在", user_record);
            let payload = format!(
                "{{\"OPENSTA\": \"{}\", \"USER\": \"{}\",\"DEVICE_ID\": \"{}\",\"DEVICE_INFO\": \"{}\"}}",
                control_type, user_name, device_id,device_info
            );

            // 开启监听mqtt指定端口
            match mqtt_client
                .publish_with_confirmation(&topic, QoS::AtLeastOnce, &payload)
                .await
            {
                Ok(_) => {
                    // 确认消息已发送
                    info!("消息已发送: {}", payload);
                }
                Err(e) => {
                    error!("MQTT 发布失败: {}", e);
                    return Resp::err(AppError::InternalServerError("MQTT 发布失败".into()))
                        .to_json_result();
                }
            }
        }
        Ok(_) => {
            //
            info!("用户名 '{}' 不存在", user_name);
        }
        Err(e) => {
            error!("检查用户名时发生错误: {}", e);
            return Resp::err(AppError::InternalServerError("服务器内部错误".into()))
                .to_json_result();
        }
    }

    // 如果需要，使用 mqtt 发送消息
    Resp::ok("", "控制命令已发送").to_json_result()
}

// 导出文件
pub async fn export_sluice_data(
    db: web::Data<DatabaseConnection>,
    query: Query<TimeQuery>,
) -> Result<HttpResponse, Error> {
    // 获取时间戳范围
    let start_time = query
        .start_time
        .unwrap_or_else(|| (Utc::now() - Duration::hours(1)).timestamp());
    let end_time = query.end_time.unwrap_or_else(|| Utc::now().timestamp());
    let topic = query.topic.clone();
    // 查询数据
    let (data, total) =
        match SluiceDataEntity::find_with_user(db.as_ref(), start_time, end_time, &topic).await {
            Ok(sluice) => {
                let total = sluice.len() as u64;
                log::info!("total2: {}, ", total);
                (sluice, total)
            }
            Err(e) => {
                error!("Error: {}", e);
                (vec![], 0)
            }
        };
    log::info!("total2: {}, ", total);
    // let data = filter_value(data, vec!["id"]);

    let sluice = SluiceDataEntity::find()
        .filter(sluice_data::Column::SluiceId.eq(topic))
        .filter(sluice_data::Column::Cstamp.gte(start_time))
        .filter(sluice_data::Column::Cstamp.lte(end_time))
        .order_by_desc(sluice_data::Column::Id)
        .all(db.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(format!("数据库操作失败: {}", e)))?;
    let total = sluice.len() as u64;
    info!("total1: {}, ", total);
    // 生成 Excel 文件
    let excel_bytes = generate_excel(&data)
        .map_err(|e| AppError::InternalServerError(format!("Excel 生成失败: {}", e)))?;
    let stream = futures::stream::once(async move { Ok(web::Bytes::from(excel_bytes)) });

    Ok(HttpResponse::Ok()
        .append_header((
            header::CONTENT_TYPE,
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        ))
        .append_header((
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}.xlsx\"", "水位历史数据"),
        ))
        .streaming::<_, Error>(stream))
}
