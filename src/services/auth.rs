use crate::common::CommonResponse;
use crate::dto::user::{LoginRequest, RegisterResponse};
use crate::error::error::AppError;
use crate::jsonwebtoken::TokenClaims;
use crate::models::user::{self, Entity as UserEntity, Model};
use crate::permission::Permission;
use crate::permission::{PERMISSION_LIST, PERMISSION_MAP};
use actix_web::{web, HttpResponse, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use log::info;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub user_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub permissions: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub id: i32,
    pub uuid: String,
}
#[derive(Debug, Serialize)]
pub struct DataResponse<T> {
    data: T,
}

#[derive(Debug, Serialize)]
pub struct LoginData {
    pub user: UserInfo,
    pub access_token: String,
    pub expires_in: u64,
}
pub async fn login(
    db: web::Data<DatabaseConnection>,
    user_data: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    user_data.validate().map_err(|e| {
        info!("{:?}", e); // 打印验证错误
        AppError::DeserializeError(e.to_string())
    })?;
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
        return Err(AppError::Unauthorized("用户名或密码错误".into()));
    }

    // 生成JWT令牌
    let token = generate_jwt(
        &credentials,
        "secret_key",
        3600, // 可配置的过期时间
    )?;
    // 如果验证通过，返回成功响应
    Ok(HttpResponse::Ok().json(build_login_response(credentials, token)))
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

fn build_login_response(credentials: Model, token: String) -> CommonResponse<LoginData> {
    let user_info = UserInfo {
        id: credentials.id,
        uuid: credentials.uuid,
        user_name: credentials.user_name,
        created_at: credentials.created_at,
        updated_at: credentials.updated_at,
        email: credentials.email,
        phone: credentials.phone,
        role: credentials.role,
        permissions: credentials.permissions.clone(),
    };

    CommonResponse {
        code: 200,
        message: "登录成功".to_string(),
        data: LoginData {
            user: user_info,
            access_token: token,
            expires_in: 3600,
        },
    }
}

pub async fn register(
    db: web::Data<DatabaseConnection>,
    user_data: web::Json<RegisterResponse>,
) -> Result<HttpResponse, AppError> {
    user_data.validate().map_err(|e| {
        info!("注册:{:?}", e); // 打印验证错误
        AppError::DeserializeError(e.to_string())
    })?;
    let user_data = user_data.into_inner(); // 提取内部数据

    // 检查用户名是否已存在
    let exists = UserEntity::find()
        .filter(user::Column::UserName.eq(&user_data.user_name))
        .count(db.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(format!("检查用户名时发生错误: {}", e)))?
        > 0;

    if exists {
        return Err(AppError::Conflict(format!(
            "用户名 '{}' 已存在",
            user_data.user_name
        )));
    }

    // 密码加密
    let hashed_password = match hash(&user_data.pass_word, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return Err(AppError::InternalServerError("密码加密失败".to_string())),
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

    let created_user = new_user
        .insert(db.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(format!("创建用户失败: {}", e)))?;

    Ok(HttpResponse::Created().json(created_user))
}

#[derive(Serialize)]
struct PermissionResponse {
    data: Vec<String>,
    code: u16,
}

#[derive(Deserialize)]
pub struct PermissionRequest {
    #[serde(default = "default_permissions")]
    permissions: String,
}

fn default_permissions() -> String {
    "0".to_string()
}
// 解析权限ID并返回权限信息
pub async fn get_permissions_by_id(
    _db: web::Data<DatabaseConnection>,
    query: web::Query<PermissionRequest>,
) -> Result<HttpResponse> {
    log::info!("permission_id: {}", query.permissions);
    match query.permissions.parse::<u64>() {
        Ok(permissions_bits) => {
            let stored_permissions =
                Permission::from_bits(permissions_bits).unwrap_or(Permission::NONE);
            let permission_names = stored_permissions
                .iter_names()
                .map(|(name, _)| name.to_string())
                .collect::<Vec<_>>();

            let response = PermissionResponse {
                data: permission_names,
                code: 200,
            };

            Ok(HttpResponse::Ok().json(response))
        }
        Err(_) => {
            let response = PermissionResponse {
                data: vec![],
                code: 400,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

pub async fn get_permissions() -> Result<HttpResponse, AppError> {
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

    Ok(HttpResponse::Ok().json(permission_list))
}
