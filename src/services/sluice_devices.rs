use crate::common::{
    PaginatedResponse, PaginationInfo, PaginationQuery, DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE,
};
use crate::dto::sluice::{SluiceDevicesRequest, UpdateSluiceDevicesRequest};
use crate::middleware::helpers::Resp;
use crate::middleware::helpers::SimpleResp;
use crate::models::sluice_devices::{self, Entity as SluiceDevicesEntity};
use crate::mqtt::{DeviceUpdate, MqttClient};
use crate::utils::query_parameter::Query;
use crate::AppError;
use actix_web::web::{self};
use chrono::Utc;
use log::*;

use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait as _, PaginatorTrait, QueryOrder,
    QuerySelect, Set,
};
use validator::Validate;
pub async fn create_sluice_devices(
    db: web::Data<DatabaseConnection>,
    params: web::Json<SluiceDevicesRequest>,
    mqtt_client: web::Data<MqttClient>,
) -> SimpleResp {
    let subscribe_topic = params.subscribe_topic.clone();
    let publish_topic = params.publish_topic.clone();
    let device_name = params.device_name.clone();
    let remark = params.remark.clone();
    let sluice_device = sluice_devices::ActiveModel {
        subscribe_topic: Set(subscribe_topic.to_string()),
        publish_topic: Set(publish_topic.to_string()),
        device_name: Set(device_name.to_string()),
        remark: Set(remark.map(|s| s.to_string())),
        create_time: Set(Utc::now().timestamp()),
        ..Default::default()
    };

    match sluice_device.insert(db.as_ref()).await {
        Ok(created_user) => {
            mqtt_client
                .notify_device_update(DeviceUpdate::Added(created_user.clone()))
                .await;
            Resp::ok(created_user, "创建设备成功").to_json_result()
        }
        Err(e) => {
            error!("创建设备失败: {}", e);
            Resp::err(AppError::InternalServerError("服务器内部错误".into())).to_json_result()
        }
    }
}
// 查询设备列表
pub async fn get_sluice_devices(
    db: web::Data<DatabaseConnection>,
    query: Query<PaginationQuery>,
) -> SimpleResp {
    // 验证分页参数
    let validated_query = match query.validate() {
        Ok(_) => query.into_inner(),
        Err(e) => {
            log::error!("分页参数验证失败: {:?}", e);

            return Resp::err(AppError::BadRequest("分页参数验证失败".to_string()))
                .to_json_result();
        }
    };
    let page = validated_query.page.unwrap_or(1);
    let limit = validated_query.limit.unwrap_or(DEFAULT_PAGE_SIZE);

    // 限制每页数量的范围
    let limit = if limit == 0 {
        DEFAULT_PAGE_SIZE
    } else if limit > MAX_PAGE_SIZE {
        MAX_PAGE_SIZE
    } else {
        limit
    };
    let offset = (page - 1) * limit;

    // 获取总数和分页数据
    let (total, sluice_devices) = tokio::try_join!(
        SluiceDevicesEntity::find().count(db.as_ref()),
        SluiceDevicesEntity::find()
            .order_by_desc(sluice_devices::Column::Id)
            .offset(Some(offset))
            .limit(Some(limit))
            .all(db.as_ref())
    )
    .map_err(|e| AppError::InternalServerError(format!("数据库操作失败: {}", e)))?;
    let total_pages = (total + limit - 1) / limit; // 整数除法避免浮点误差

    info!("total1: {}, users1: {:?}, ", total, sluice_devices);
    // let data = deep_filter_data(users, vec!["pass_word"]);
    // 获取分页用户数据
    let response = PaginatedResponse {
        data: sluice_devices,
        pagination: PaginationInfo {
            total,
            total_pages,
            current_page: page,
            limit,
            has_next: page < total_pages,
            has_previous: page > 1,
        },
    };

    Resp::ok(response, "获取用户列表成功").to_json_result()
}
// 删除当前设备
pub async fn delete_sluice_devices(
    db: web::Data<DatabaseConnection>,
    id: web::Path<u32>,
    mqtt_client: web::Data<MqttClient>,
) -> SimpleResp {
    let id = id.into_inner();
    let deleted = SluiceDevicesEntity::find_by_id(id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(format!("数据库操作失败: {}", e)))?;
    match deleted {
        Some(sluice_device) => {
            let delete_result = sluice_device.clone().delete(db.as_ref()).await;
            match delete_result {
                Ok(_) => {
                    mqtt_client
                        .notify_device_update(DeviceUpdate::Removed(sluice_device.clone()))
                        .await;
                    Resp::ok((), "删除设备成功").to_json_result()
                }
                Err(e) => {
                    error!("删除设备失败: {}", e);
                    Resp::err(AppError::InternalServerError("服务器内部错误".into()))
                        .to_json_result()
                }
            }
        }
        None => Resp::err(AppError::NotFound("设备不存在".into())).to_json_result(),
    }
}

// 更新设备
pub async fn update_sluice_devices(
    db: web::Data<DatabaseConnection>,
    id: web::Path<u32>,
    params: web::Json<UpdateSluiceDevicesRequest>,
    mqtt_client: web::Data<MqttClient>,
) -> SimpleResp {
    let sluice_device = match SluiceDevicesEntity::find_by_id(*id).one(db.as_ref()).await {
        Ok(u) => u,
        Err(e) => {
            error!("获取设备信息失败: {}", e); // 记录错误日志
            return Resp::err(AppError::InternalServerError(
                "获取设备信息失败".to_string(),
            ))
            .to_json_result();
        }
    };

    let existing_sluice_device =
        sluice_device.ok_or_else(|| AppError::NotFound(format!("ID为{}的设备不存在", id)))?;
    // 3. 准备更新模型
    let mut sluice_device_active: sluice_devices::ActiveModel = existing_sluice_device.into();

    if let Some(subscribe_topic) = &params.subscribe_topic {
        sluice_device_active.subscribe_topic = Set(subscribe_topic.to_string());
    }
    if let Some(publish_topic) = &params.publish_topic {
        sluice_device_active.publish_topic = Set(publish_topic.to_string());
    }
    if let Some(device_name) = &params.device_name {
        sluice_device_active.device_name = Set(device_name.to_string());
    }
    if let Some(remark) = &params.remark {
        sluice_device_active.remark = Set(Some(remark.to_string()));
    }
    if let Some(latitude) = &params.latitude {
        sluice_device_active.latitude = Set(Some(latitude.to_string()));
    }

    if let Some(longitude) = &params.longitude {
        sluice_device_active.longitude = Set(Some(longitude.to_string()));
    }

    // sluice_device_active.remark = Set(remark.map(|s| s.to_string()));
    match sluice_device_active.update(db.as_ref()).await {
        Ok(updated_sluice_device) => {
            mqtt_client
                .notify_device_update(DeviceUpdate::Updated(updated_sluice_device.clone()))
                .await;
            Resp::ok(updated_sluice_device, "更新设备成功").to_json_result()
        }
        Err(e) => {
            error!("更新设备失败: {}", e);
            Resp::err(AppError::InternalServerError("服务器内部错误".into())).to_json_result()
        }
    }
}
