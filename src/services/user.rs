use crate::common::{CommonResponse, PaginationInfo};
use crate::config::permission::{Permission, PERMISSION_MAP};
use crate::data_processing::{deep_filter_data, filter_value};
use crate::dto::user::{UpdateUserRequest, UserDto};
use crate::error::error::AppError;
use crate::middleware::helpers::{Resp, SimpleResp};
use crate::models::user::{self, Entity as UserEntity};
use crate::utils::query_parameter::Query;
use crate::utils::sse::SseNotifier;
use actix_web::web;
use chrono::Utc;
use log::{error, info, warn};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Debug;
use utoipa::ToSchema;
use uuid::Uuid; // 添加uuid crate依赖
use validator::Validate;

const DEFAULT_PAGE_SIZE: u64 = 10;
const MAX_PAGE_SIZE: u64 = 100;
#[derive(Validate, Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    #[validate(range(min = 1, message = "页码必须大于1"))]
    pub page: Option<u64>,
    // 每页数量不能超过100
    #[serde(default = "default_size")]
    #[validate(range(max = MAX_PAGE_SIZE, message = "每页数量不能超过100"))]
    pub limit: Option<u64>,
}
// 默认值函数
// 添加默认函数实现
fn default_page() -> Option<u64> {
    Some(1)
}

fn default_size() -> Option<u64> {
    Some(DEFAULT_PAGE_SIZE)
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    data: Vec<T>,
    pagination: PaginationInfo,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
#[derive(Serialize, ToSchema)]
pub struct Users {
    pub id: i32,
    pub uuid: String,
    pub user_name: String,
    pub email: Option<String>,
    pub image: Option<String>,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub permissions: Option<String>,
    pub binding: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    data: Vec<Users>,
    pagination: PaginationInfo,
}

#[utoipa::path(
    get,
    path = "/api/users",
    request_body = PaginationQuery,
    tag = "用户模块",
    operation_id = "获取用户列表",
    responses(
        (status = 200, description = "获取用户列表成功", body = CommonResponse<UserResponse>),
    ),
)]
// 获取用户列表（带分页）
pub async fn get_all_users(
    db: web::Data<DatabaseConnection>,
    query: Query<PaginationQuery>,
) -> SimpleResp {
    // 验证分页参数
    let validated_query = match query.validate() {
        Ok(_) => query.into_inner(),
        Err(e) => {
            log::error!("分页参数验证失败: {:?}", e);
            let error_response = ErrorResponse {
                error: e.to_string(),
            };
            return Resp::ok("分页参数验证失败", &error_response.error).to_json_result();
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
    let (total, users) = tokio::try_join!(
        UserEntity::find().count(db.as_ref()),
        UserEntity::find()
            .order_by_desc(user::Column::Id)
            .offset(Some(offset))
            .limit(Some(limit))
            .all(db.as_ref())
    )
    .map_err(|e| AppError::InternalServerError(format!("数据库操作失败: {}", e)))?;
    let total_pages = (total + limit - 1) / limit; // 整数除法避免浮点误差

    info!("total1: {}, users1: {:?}, ", total, users);
    let data = deep_filter_data(users, vec!["pass_word"]);
    // 获取分页用户数据
    let response = PaginatedResponse {
        data,
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

#[utoipa::path(
    get,
    path = "/api/users/{uuid}",
    request_body = String,
    tag = "用户模块",
    operation_id = "获取指定用户信息",
    responses(
        (status = 200, description = "获取用户信息成功", body = CommonResponse<Users>),
        (status = 404, description = "用户不存在", body = CommonResponse<Option<Users>>)
    ),
)]
// 通过uuID获取用户
pub async fn get_user_by_uuid(
    db: web::Data<DatabaseConnection>,
    uuid: web::Path<String>,
) -> SimpleResp {
    // 验证UUID格式
    let uuid =
        Uuid::parse_str(&uuid).map_err(|_| AppError::BadRequest("无效的UUID格式".to_string()))?;

    // 查询用户
    let user = UserEntity::find_by_uuid(&uuid.to_string())
        .one(db.as_ref())
        .await
        .map_err(|e| {
            error!("获取用户信息失败: {}", e); // 记录错误日志
            AppError::InternalServerError("获取用户信息失败".to_string())
        })?;

    match user {
        Some(user) => {
            let data = filter_value(user.into(), vec!["pass_word"]);
            Resp::ok(data, "获取用户信息成功").to_json_result()
        }
        None => {
            Resp::err(AppError::NotFound(format!("UUID为{}的用户不存在", uuid))).to_json_result()
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/users/{uuid}",
    request_body = UserDto,
    operation_id = "更新用户信息",
    params(
        ("uuid" = String, Path, description = "用户的 UUID")
    ),
    responses(
        (status = 200, description = "用户信息更新成功", body = CommonResponse<Users>),
        (status = 400, description = "请求参数错误", body = AppError),
        (status = 404, description = "用户不存在", body = AppError),
        (status = 409, description = "用户名已存在", body = AppError),
        (status = 500, description = "服务器内部错误", body = AppError)
    ),
    security(),
    tag = "用户模块"
)]
// 更新用户信息
pub async fn update_user(
    db: web::Data<DatabaseConnection>,
    uuid: web::Path<String>,
    user_data: web::Json<UpdateUserRequest>,
    notifier: web::Data<SseNotifier>,
) -> SimpleResp {
    // 验证UUID格式
    let uuid =
        Uuid::parse_str(&uuid).map_err(|_| AppError::BadRequest("无效的UUID格式".to_string()))?;

    // 2. 获取现有用户
    let existing_user = UserEntity::find_by_uuid(&uuid.to_string())
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
            .filter(user::Column::Uuid.ne(&*uuid.to_string()))
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

    let data = filter_value(updated_user.into(), vec!["pass_word"]);
    Resp::ok(data, "修改用户信息成功").to_json_result()
}

#[utoipa::path(
    delete,
    path = "/api/users/{uuid}",
    params(
        ("uuid" = String, Path, description = "用户的 UUID")
    ),
    responses(
        (status = 200, description = "用户删除成功", body = UserDto),
        (status = 400, description = "请求参数错误", body = AppError),
        (status = 404, description = "用户不存在", body = AppError),
        (status = 500, description = "服务器内部错误", body = AppError)
    ),
    security(),
    tag = "用户模块",
    operation_id = "删除用户",

)]

// 删除用户
pub async fn delete_user(db: web::Data<DatabaseConnection>, uuid: web::Path<String>) -> SimpleResp {
    info!("删除用户请求: {}", uuid); // 改为info级别

    // 使用uuid库验证UUID格式
    let uuid =
        Uuid::parse_str(&uuid).map_err(|_| AppError::BadRequest("无效的UUID格式".to_string()))?;

    let delete_result = UserEntity::delete_by_uuid(db.as_ref(), &uuid.to_string())
        .await
        .map_err(|e| {
            error!("删除用户时数据库错误: {}", e); // 记录详细错误日志
            AppError::InternalServerError("删除用户失败".to_string()) // 对外暴露简略信息
        })?;

    if delete_result.rows_affected == 0 {
        Err(AppError::NotFound(format!("UUID为{}的用户不存在", uuid)))
    } else {
        info!("成功删除用户: {}", uuid); // 记录成功操作
        Resp::ok("", &format!("用户 {} 已删除", uuid).to_string()).to_json_result()
    }
}
