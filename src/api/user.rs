use crate::error::error::AppError;
use crate::models::user::{self, Entity as UserEntity};
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
const DEFAULT_PAGE_SIZE: u64 = 10;
const MAX_PAGE_SIZE: u64 = 100;

#[derive(Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    page: u64,
    #[serde(default = "default_limit")]
    limit: u64,
}

fn default_page() -> u64 {
    1
}

fn default_limit() -> u64 {
    DEFAULT_PAGE_SIZE
}

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

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    user_name: String,
    email: String,
    age: Option<i32>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// 获取用户列表（带分页）
pub async fn get_all_users(
    db: web::Data<DatabaseConnection>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse> {
    // 验证分页参数
    if query.page == 0 {
        let error_response = ErrorResponse {
            error: "页码必须大于0".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }

    // 限制每页数量的范围
    let limit = if query.limit == 0 {
        DEFAULT_PAGE_SIZE
    } else if query.limit > MAX_PAGE_SIZE {
        MAX_PAGE_SIZE
    } else {
        query.limit
    };

    // 计算偏移量
    let offset = (query.page - 1) * limit;

    // 获取用户总数
    let total = UserEntity::find()
        .count(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("获取用户总数失败: {}", e)))?;

    // 计算总页数
    let total_pages = (total as f64 / limit as f64).ceil() as u64;

    // 获取分页用户数据
    let users = UserEntity::find()
        .order_by_asc(user::Column::Id)
        .offset(Some(offset))
        .limit(Some(limit))
        .all(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("获取用户列表失败: {}", e)))?;

    let response = PaginatedResponse {
        data: users,
        pagination: PaginationInfo {
            total,
            total_pages,
            current_page: query.page,
            limit,
            has_next: query.page < total_pages,
            has_previous: query.page > 1,
        },
    };

    Ok(HttpResponse::Ok().json(response))
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
        .map_err(|e| AppError::Internal(format!("获取用户信息失败: {}", e)))?;

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

// 更新用户
pub async fn update_user(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
    user_data: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse> {
    // 验证ID
    if *id <= 0 {
        let error_response = ErrorResponse {
            error: "用户ID必须大于0".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }

    // 验证用户名不为空
    if user_data.user_name.trim().is_empty() {
        let error_response = ErrorResponse {
            error: "用户名不能为空".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }

    // 验证邮箱不为空
    if user_data.email.trim().is_empty() {
        let error_response = ErrorResponse {
            error: "邮箱不能为空".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }

    // 获取现有用户
    let existing_user = UserEntity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("检查用户信息失败: {}", e)))?;

    let existing_user = match existing_user {
        Some(user) => user,
        None => {
            let error_response = ErrorResponse {
                error: format!("ID为{}的用户不存在", id),
            };
            return Ok(HttpResponse::NotFound().json(error_response));
        }
    };

    // 检查用户名是否被其他用户占用
    if existing_user.user_name != user_data.user_name {
        let username_exists = UserEntity::find()
            .filter(user::Column::UserName.eq(&user_data.user_name))
            .filter(user::Column::Id.ne(*id)) // 排除当前用户
            .count(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("检查用户名时发生错误: {}", e)))?
            > 0;

        if username_exists {
            let error_response = ErrorResponse {
                error: format!("用户名 '{}' 已存在", user_data.user_name),
            };
            return Ok(HttpResponse::Conflict().json(error_response));
        }
    }

    // 创建更新模型
    let mut user_active: user::ActiveModel = existing_user.into();

    // 更新字段
    user_active.user_name = Set(user_data.user_name.clone());
    // user_active.email = Set(user_data.email.clone());
    user_active.age = Set(user_data.age);

    // 如果需要更新时间戳
    user_active.updated_at = Set(Utc::now());

    // 执行更新
    let updated_user = user_active
        .update(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("更新用户信息失败: {}", e)))?;

    Ok(HttpResponse::Ok().json(updated_user))
}
// 删除用户
pub async fn delete_user(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    // 验证ID
    if *id <= 0 {
        let error_response = ErrorResponse {
            error: "用户ID必须大于0".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }

    // 执行删除并检查结果
    let delete_result = UserEntity::delete_by_id(*id)
        .exec(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("删除用户失败: {}", e)))?;

    // 检查是否成功删除
    if delete_result.rows_affected == 0 {
        let error_response = ErrorResponse {
            error: format!("ID为{}的用户不存在", id),
        };
        return Ok(HttpResponse::NotFound().json(error_response));
    }

    // 成功删除，返回200 OK并附带成功消息
    Ok(HttpResponse::Ok().json(json!({
        "message": format!("用户ID {} 已成功删除", *id)
    })))
}
