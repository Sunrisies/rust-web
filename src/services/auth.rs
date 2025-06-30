use crate::common::CommonResponse;
use crate::dto::user::{LoginRequest, RegisterResponse, UserDto};
use crate::error::error::AppError;
use crate::jsonwebtoken::TokenClaims;
use crate::middleware::helpers::{Resp, SimpleResp};
use crate::models::user::{self, Entity as UserEntity, Model};
use crate::permission::Permission;
use crate::permission::{PERMISSION_LIST, PERMISSION_MAP};
use crate::services::user::UserInfo;
use actix_web::{web, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use log::{error, info, warn};
use sea_orm::{
    entity::prelude::*, ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection,
    EntityTrait, PaginatorTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use utoipa::ToSchema;
use validator::Validate;

// #[derive(Debug, Serialize, ToSchema)]
// pub struct UserInfo {
//     pub user_name: String,
//     pub email: Option<String>,
//     pub phone: Option<String>,
//     pub role: Option<String>,
//     pub permissions: Option<String>,
//     pub created_at: DateTimeUtc,
//     pub updated_at: DateTimeUtc,
//     pub id: i32,
//     pub uuid: String,
// }
#[derive(Debug, Serialize)]
pub struct DataResponse<T> {
    data: T,
}

#[derive(Serialize, ToSchema)]
pub struct LoginData {
    pub user: UserInfo,
    pub access_token: String,
    pub expires_in: u64,
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    tag = "鉴权模块",
    operation_id = "用户登录",
    responses(
        (status = 200, description = "登录成功", body = CommonResponse<LoginData>),
        (status = 400, description = "验证错误", body = SimpleRespData),
        (status = 404, description = "用户名不存在", body = SimpleRespData),
        (status = 500, description = "服务器内部错误", body = SimpleRespData),
    ),
)]
pub async fn login(
    db: web::Data<DatabaseConnection>,
    user_data: web::Json<LoginRequest>,
) -> SimpleResp {
    match user_data.validate() {
        Ok(_) => {}
        Err(e) => {
            info!("{:?}", e); // 打印验证错误
            return Resp::err(AppError::DeserializeError(e.to_string())).to_json_result();
        }
    };
    let user_data = user_data.into_inner(); // 提取内部数据

    // 检查用户名是否存在
    let credentials = UserEntity::find()
        .filter(user::Column::UserName.eq(&user_data.user_name))
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(format!("检查用户名时发生错误: {}", e)))?
        .ok_or(AppError::NotFound(format!(
            "用户名 '{}' 不存在",
            user_data.user_name
        )))?;

    // 验证密码（使用 bcrypt 验证加密后的密码）
    if let Err(_) = verify(&user_data.pass_word, &credentials.pass_word) {
        return Resp::err(AppError::Unauthorized("用户名或密码错误".into())).to_json_result();
    }

    // 生成JWT令牌
    let token = generate_jwt(
        &credentials,
        "secret_key",
        3600, // 可配置的过期时间
    )?;
    Resp::ok(build_login_response(credentials, token), "登录成功").to_json_result()
}

// 提取JWT生成逻辑
fn generate_jwt(credentials: &Model, secret: &str, expires_in: u64) -> Result<String, AppError> {
    let exp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + expires_in; // 令牌有效期为 1 小时

    let token_claims = TokenClaims {
        user_uuid: credentials.uuid.clone(),
        user_name: credentials.user_name.clone(),
        exp: exp as usize,
        permissions: credentials.permissions.clone(),
    };

    encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| {
        log::error!("JWT生成失败: {}", e);
        AppError::InternalServerError("登录服务暂时不可用".into())
    })
}

fn build_login_response(credentials: Model, token: String) -> LoginData {
    let user_info = UserInfo {
        id: credentials.id,
        uuid: credentials.uuid,
        user_name: credentials.user_name,
        created_at: credentials.created_at.to_string(),
        updated_at: credentials.updated_at.to_string(),
        email: credentials.email,
        phone: credentials.phone,
        role: credentials.role,
        permissions: credentials.permissions.clone(),
        image: credentials.image,
        binding: credentials.binding,
    };

    LoginData {
        user: user_info,
        access_token: token,
        expires_in: 3600,
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SimpleRespData {
    data: String,
    message: String,
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterResponse,
    tag = "鉴权模块",
    operation_id = "用户注册",
    responses(
        (status = 200, description = "注册成功", body = CommonResponse<UserDto>),
        (status = 400, description = "验证错误", body = CommonResponse<Option<UserDto>>),
        (status = 409, description = "用户名已存在", body = CommonResponse<Option<UserDto>>),
        (status = 500, description = "服务器内部错误", body = SimpleRespData),
    ),
)]

pub async fn register(
    db: web::Data<DatabaseConnection>,
    user_data: web::Json<RegisterResponse>,
) -> SimpleResp {
    if let Err(e) = user_data.validate() {
        info!("注册:{:?}", e); // 打印验证错误
        return Resp::err(AppError::DeserializeError(e.to_string())).to_json_result();
    }
    let user_data = user_data.into_inner(); // 提取内部数据

    // 检查用户名是否已存在
    match UserEntity::find()
        .filter(user::Column::UserName.eq(&user_data.user_name))
        .count(db.as_ref())
        .await
    {
        Ok(count) if count > 0 => {
            warn!("用户名 '{}' 已存在", user_data.user_name);
            return Resp::err(AppError::Conflict("用户名已存在".into())).to_json_result();
        }
        Ok(_) => {}
        Err(e) => {
            error!("检查用户名时发生错误: {}", e);
            return Resp::err(AppError::InternalServerError("服务器内部错误".into()))
                .to_json_result();
        }
    }

    // 密码加密
    let hashed_password = match hash(&user_data.pass_word, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(e) => {
            log::error!("密码加密失败: {}", e); // 详细错误日志记录在服务器端
            return Resp::err(AppError::InternalServerError("密码加密失败".into()))
                .to_json_result();
        }
    };

    // 创建新用户
    let new_user = user::ActiveModel {
        uuid: Set(Uuid::new_v4().to_string()),
        user_name: Set(user_data.user_name.clone()),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        permissions: Set(Some(Permission::ALL.bits().to_string())), // 设置默认权限
        pass_word: Set(hashed_password.clone()), // 注意：这里应该存储哈希后的密码
        ..Default::default()
    };

    match new_user.insert(db.as_ref()).await {
        Ok(created_user) => Resp::ok(created_user, "注册成功").to_json_result(),
        Err(e) => {
            error!("创建用户失败: {}", e);
            Resp::err(AppError::InternalServerError("服务器内部错误".into())).to_json_result()
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct PermissionDto {
    #[serde(default = "default_permissions")]
    permissions: String,
}

fn default_permissions() -> String {
    "0".to_string()
}
#[utoipa::path(
    get,
    path = "/api/auth/permission",
    request_body = PermissionDto,
    tag = "鉴权模块",
    operation_id = "获取指定用户权限",
    responses(
        (status = 200, description = "获取权限成功", body = SimpleRespData),
        (status = 400, description = "权限ID格式错误", body = SimpleRespData),
    ),
)]
// 解析权限ID并返回权限信息
pub async fn get_permissions_by_id(query: web::Query<PermissionDto>) -> SimpleResp {
    info!("permission_id: {}", query.permissions);
    match query.permissions.parse::<u64>() {
        Ok(permissions_bits) => {
            let stored_permissions =
                Permission::from_bits(permissions_bits).unwrap_or(Permission::NONE);
            let permission_names = stored_permissions
                .iter_names()
                .map(|(name, _)| name.to_string())
                .collect::<Vec<_>>();
            Resp::ok(permission_names, "获取权限成功").to_json_result()
        }
        Err(_) => Resp::err(AppError::BadRequest("权限ID格式错误".into())).to_json_result(),
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PermissionResponse {
    name: String,
    description: String,
}

#[utoipa::path(
    get,
    path = "/api/auth/permissions",
    request_body = (),
    tag = "鉴权模块",
    operation_id = "获取权限列表",
    responses(
        (status = 200, description = "获取权限列表成功", body = CommonResponse<Vec<PermissionResponse>>),
    ),
)]
pub async fn get_permissions() -> SimpleResp {
    let permission_list = PERMISSION_LIST
        .iter()
        .filter(|(name, _)| {
            if let Some(perm) = PERMISSION_MAP.get(*name) {
                // 判断是否是单一权限（二进制表示中只有一个1）
                perm.bits().count_ones() == 1
            } else {
                false
            }
        })
        .map(|(name, description)| {
            serde_json::json!({
                "name": name,
                "description": description
            })
        })
        .collect::<Vec<_>>();
    Resp::ok(permission_list, "获取权限列表成功").to_json_result()
}
