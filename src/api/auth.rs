use crate::common::CommonResponse;
use crate::dto::user::{LoginRequest, RegisterResponse};
use crate::error::error::AppError;
use crate::models::user::{self, Entity as UserEntity};
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
use serde::Serialize;
use std::time::SystemTime;
use validator::Validate;
#[derive(Debug, Serialize)]
pub struct TokenClaims {
    pub user_uuid: String,
    pub user_name: String,
    pub exp: usize, // 令牌过期时间
}
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
        .map_err(|e| AppError::Internal(format!("检查用户名时发生错误: {}", e)))?
        .ok_or(AppError::NotFound(format!(
            "用户名 '{}' 不存在",
            user_data.user_name
        )))?;

    // 验证密码（使用 bcrypt 验证加密后的密码）
    if let Err(_) = verify(&user_data.pass_word, &credentials.pass_word) {
        return Err(AppError::Unauthorized("用户名或密码错误".into()));
    }
    // 生成 JWT 令牌
    let exp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600; // 令牌有效期为 1 小时

    let token_claims = TokenClaims {
        user_uuid: credentials.uuid.clone(),
        user_name: credentials.user_name.clone(),
        exp: exp as usize,
    };

    let token = encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret("secret_key".as_bytes()), // 替换为你的密钥
    )
    .map_err(|e| AppError::Internal(format!("生成令牌时发生错误: {}", e)))?;
    let user_info = UserInfo {
        id: credentials.id,
        uuid: credentials.uuid,
        user_name: credentials.user_name,
        created_at: credentials.created_at,
        updated_at: credentials.updated_at,
        email: credentials.email,
        phone: credentials.phone,
        role: credentials.role,
        permissions: credentials.permissions,
    };
    // 构造返回的数据结构
    let login_response = CommonResponse {
        code: 200,
        message: "登录成功".to_string(),
        data: LoginData {
            user: user_info,
            access_token: token,
            expires_in: 3600,
        },
    };

    // 如果验证通过，返回成功响应
    Ok(HttpResponse::Ok().json(login_response))
}
pub async fn register(
    db: web::Data<DatabaseConnection>,
    user_data: web::Json<RegisterResponse>,
) -> Result<HttpResponse, AppError> {
    user_data.validate().map_err(|e| {
        info!("{:?}", e); // 打印验证错误
        AppError::DeserializeError(e.to_string())
    })?;
    let user_data = user_data.into_inner(); // 提取内部数据

    // 检查用户名是否已存在
    let exists = UserEntity::find()
        .filter(user::Column::UserName.eq(&user_data.user_name))
        .count(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("检查用户名时发生错误: {}", e)))?
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
        Err(_) => return Err(AppError::Internal("密码加密失败".to_string())),
    };

    // 创建新用户
    let new_user = user::ActiveModel {
        uuid: Set(Uuid::new_v4().to_string()),
        user_name: Set(user_data.user_name.clone()),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        pass_word: Set(hashed_password.clone()), // 注意：这里应该存储哈希后的密码
        ..Default::default()
    };

    let created_user = new_user
        .insert(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("创建用户失败: {}", e)))?;

    Ok(HttpResponse::Created().json(created_user))
}
