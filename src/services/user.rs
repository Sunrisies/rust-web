use std::fmt::Debug;

use crate::config::permission::{Permission, PERMISSION_MAP};
use crate::dto::user::UpdateUserRequest;
use crate::error::error::AppError;
use crate::middleware::helpers::{Resp, SimpleResp};
use crate::models::user::{self, Entity as UserEntity};
use crate::utils::sse::SseNotifier;
use actix_web::cookie::time::error;
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;
const DEFAULT_PAGE_SIZE: u64 = 10;
const MAX_PAGE_SIZE: u64 = 100;
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaginationQuery {
    #[validate(range(min = 1, message = "页码必须大于01"))]
    pub page: u64,
    // 每页数量不能超过100
    #[validate(range(max = MAX_PAGE_SIZE, message = "每页数量不能超过100"))]
    pub limit: u64,
}

// impl PaginationQuery {
//     // 获取处理后的分页参数（应用默认值）
//     pub fn get_params(&self) -> (u64, u64) {
//         let page = self.page.unwrap_or(1);
//         let limit = self.limit.unwrap_or(10);
//         (page, limit)
//     }

//     // 可选：添加自定义验证逻辑（如互斥规则等）
//     pub fn validate_input(&self) -> Result<(), validator::ValidationErrors> {
//         self.validate()
//     }
// }

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    data: Vec<T>,
    pagination: PaginationInfo,
}

#[derive(Serialize)]
pub struct PaginationInfo {
    total: u64,
    total_pages: u64,
    current_page: u64,
    limit: u64,
    has_next: bool,
    has_previous: bool,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
42.192.53.166:21116
rx7HM207TQKXrONIDUIsCBb1V8na8jRzZuDlrYCBSs8=
// 获取用户列表（带分页）
pub async fn get_all_users(
    db: web::Data<DatabaseConnection>,
    query: web::Json<PaginationQuery>,
) -> SimpleResp {
    // if let Err(mut e) = query.validate_input() {
    //     // 提取所有错误提示
    //     let errors_str = e.errors_mut();
    //     log::error!("分页参数验证失败: {:?}", errors_str);
    //     let error_response = ErrorResponse {
    //         error: e.to_string(),
    //     };
    //     // let errors_str = e
    //     //     .iter()
    //     //     .map(|e| e.to_string())
    //     //     .collect::<Vec<String>>()
    //     //     .join(", ");

    //     return Resp::ok("分页参数验证失败", &error_response.error).to_json_result();
    //     // return Resp::err(AppError::BadRequest(error_response.error)).to_json_result();
    // }
    log::error!("触发了获取用户列表的函数");
    // let (page, limit) = query.get_params();

    // // 验证分页参数
    // let mut query_with_defaults = PaginationQuery {
    //     page: Some(page),
    //     limit: Some(limit),
    // };
    // if let Err(validation_errors) = query_with_defaults.validate() {
    //     log::error!("验证失败: {:?}", validation_errors);
    //     let error_response = ErrorResponse {
    //         error: validation_errors.to_string(),
    //     };
    //     return Resp::ok(error_response).to_json_result();
    // }
    // 限制每页数量的范围
    // let limit = if limit == 0 {
    //     DEFAULT_PAGE_SIZE
    // } else if limit > MAX_PAGE_SIZE {
    //     MAX_PAGE_SIZE
    // } else {
    //     limit
    // };

    // // 计算偏移量
    // let offset = (page - 1) * limit;

    // // 获取用户总数
    // let total = UserEntity::find()
    //     .count(db.as_ref())
    //     .await
    //     .map_err(|e| AppError::InternalServerError(format!("获取用户总数失败: {}", e)))?;

    // // 计算总页数
    // let total_pages = (total as f64 / limit as f64).ceil() as u64;

    // // 获取分页用户数据
    // let users = UserEntity::find()
    //     .order_by_asc(user::Column::Id)
    //     .offset(Some(offset))
    //     .limit(Some(limit))
    //     .all(db.as_ref())
    //     .await
    //     .map_err(|e| AppError::InternalServerError(format!("获取用户列表失败: {}", e)))?;

    // let response = PaginatedResponse {
    //     data: users,
    //     pagination: PaginationInfo {
    //         total,
    //         total_pages,
    //         current_page: page,
    //         limit,
    //         has_next: page < total_pages,
    //         has_previous: page > 1,
    //     },
    // };

    Resp::ok("response", "获取用户列表成功").to_json_result()
    // Ok(HttpResponse::Ok().json(response))
}

// 通过ID获取用户
pub async fn get_user_by_uuid(
    db: web::Data<DatabaseConnection>,
    uuid: web::Path<String>,
) -> Result<HttpResponse> {
    // 验证UUID格式
    if uuid.is_empty() || uuid.len() != 36 {
        let error_response = ErrorResponse {
            error: "无效的UUID格式".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }

    // 查询用户
    let user = UserEntity::find_by_uuid(&uuid)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(format!("获取用户信息失败: {}", e)))?;

    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => {
            let error_response = ErrorResponse {
                error: format!("UUID为{}的用户不存在", uuid),
            };
            Ok(HttpResponse::NotFound().json(error_response))
        }
    }
}
// 更新用户信息
pub async fn update_user(
    db: web::Data<DatabaseConnection>,
    uuid: web::Path<String>,
    user_data: web::Json<UpdateUserRequest>,
    // sse_notifier: web::Data<SseNotifier>, // 添加 SSE 通知器
    notifier: web::Data<SseNotifier>,
) -> Result<HttpResponse, AppError> {
    // 验证UUID格式
    if uuid.is_empty() || uuid.len() != 36 {
        let error_response = ErrorResponse {
            error: "无效的UUID格式".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }

    // 2. 获取现有用户
    let existing_user = UserEntity::find_by_uuid(&uuid)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(format!("查询用户失败: {}", e)))?;

    let existing_user =
        existing_user.ok_or_else(|| AppError::NotFound(format!("ID为{}的用户不存在", uuid)))?;

    // 3. 准备更新模型
    let mut user_active: user::ActiveModel = existing_user.into();

    // 4. 用户名更新逻辑

    if user_active.user_name != Set(user_data.user_name.clone()) {
        let exists = UserEntity::find()
            .filter(user::Column::UserName.eq(&user_data.user_name))
            .filter(user::Column::Uuid.ne(&*uuid))
            .count(db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(format!("用户名检查失败: {}", e)))?
            > 0;

        if exists {
            return Err(AppError::Conflict(format!(
                "用户名'{}'已存在",
                user_data.user_name
            )));
        }
        user_active.user_name = Set(user_data.user_name.clone());
    }

    // 5. 权限更新逻辑
    if let Some(permissions) = &user_data.permissions {
        // 使用 PERMISSION_MAP 来验证权限
        let mut permission_bits = Permission::empty();

        for perm in permissions {
            match PERMISSION_MAP.get(perm.as_str()) {
                Some(flag) => {
                    permission_bits.insert(*flag);
                }
                None => {
                    return Err(AppError::BadRequest(format!("无效权限: {}", perm)));
                }
            }
        }

        // 将权限位转换为数值存储
        user_active.permissions = Set(Some(permission_bits.bits().to_string()));
    }

    // 6. 其他字段更新
    // if let Some(image) = &user_data.image {
    //     user_active.avatar = Set(image.clone());
    // }

    // 7. 更新时间戳
    user_active.updated_at = Set(Utc::now());

    // 8. 执行更新
    let updated_user = user_active
        .update(db.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(format!("更新失败: {}", e)))?;
    let notification = serde_json::json!({
        "event": "user_updated",
        "data": {
            "user_id": updated_user.id,
            "updated_fields": {
                "username": &user_data.user_name,
                "permissions": &user_data.permissions
            }
        }
    });

    notifier.notify(&notification.to_string());
    Ok(HttpResponse::Ok().json(updated_user))
}

// 删除用户
pub async fn delete_user(
    db: web::Data<DatabaseConnection>,
    uuid: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    log::error!("触发了删除用户的函数");
    // 验证UUID格式
    if uuid.is_empty() || uuid.len() != 36 {
        return Err(AppError::BadRequest("无效的UUID格式".to_string()));
    }

    let delete_result = UserEntity::delete_by_uuid(db.as_ref(), &uuid)
        .await
        .map_err(|e| AppError::InternalServerError(format!("删除用户时出错: {}", e)))?;

    if delete_result.rows_affected == 0 {
        Err(AppError::NotFound(format!("UUID 为 {} 的用户不存在", uuid)))
    } else {
        Ok(HttpResponse::Ok().json(json!({
            "message": format!("UUID 为 {} 的用户已删除", uuid)
        })))
    }
}
