use crate::common::{
    CommonResponse, PaginatedResponse, PaginationInfo, PaginationQuery, DEFAULT_PAGE_SIZE,
    MAX_PAGE_SIZE,
};
use crate::data_processing::{deep_filter_data, filter_value};
use crate::dto::user::UpdateUserRequest;
use crate::error::error::AppError;
use crate::middleware::helpers::{Resp, SimpleResp};
use crate::models::user::{self, Entity as UserEntity};
use crate::utils::query_parameter::Query;
use crate::utils::sse::SseNotifier;
use actix_web::web;
use chrono::Utc;
use log::{error, info};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use serde::Serialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
#[derive(Serialize, ToSchema)]
pub struct UserInfo {
    pub id: i64,
    pub user_name: String,
    pub email: Option<String>,
    pub create_time: String,
    pub update_time: String,
    pub status: i8,
}

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    data: Vec<UserInfo>,
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
        (status = 200, description = "获取用户信息成功", body = CommonResponse<UserInfo>),
        (status = 404, description = "用户不存在", body = CommonResponse<Option<UserInfo>>)
    ),
)]

// 更新用户信息
pub async fn update_user(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i64>,
    user_data: web::Json<UpdateUserRequest>,
    notifier: web::Data<SseNotifier>,
) -> SimpleResp {
    let existing_user = match UserEntity::find_by_id(*id).one(db.as_ref()).await {
        Ok(u) => u,
        Err(e) => {
            error!("获取用户信息失败: {}", e); // 记录错误日志
            return Resp::err(AppError::InternalServerError(
                "获取用户信息失败".to_string(),
            ))
            .to_json_result();
        }
    };

    let existing_user =
        existing_user.ok_or_else(|| AppError::NotFound(format!("ID为{}的用户不存在", id)))?;

    // 3. 准备更新模型
    let mut user_active: user::ActiveModel = existing_user.into();

    // 4. 用户名更新逻辑

    if user_active.user_name != Set(user_data.user_name.clone()) {
        let exists = UserEntity::find()
            .filter(user::Column::UserName.eq(&user_data.user_name))
            .filter(user::Column::Id.ne(&*id.to_string()))
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

    // 7. 更新时间戳
    user_active.update_time = Set(Utc::now());

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

// 删除用户
pub async fn delete_user(db: web::Data<DatabaseConnection>, id: web::Path<i64>) -> SimpleResp {
    let delete_result = match UserEntity::delete_many()
        .filter(user::Column::Id.eq(*id))
        .exec(db.as_ref())
        .await
    {
        Ok(u) => u,
        Err(e) => {
            error!("删除用户失败: {}", e); // 记录错误日志
            return Resp::err(AppError::InternalServerError("删除用户失败".to_string()))
                .to_json_result();
        }
    };
    if delete_result.rows_affected == 0 {
        Resp::err(AppError::NotFound(format!("ID为{}的用户不存在", id))).to_json_result()
    } else {
        info!("成功删除用户: {}", id); // 记录成功操作
        Resp::ok("", &format!("用户 {} 已删除", id).to_string()).to_json_result()
    }
}
