use crate::dto::user::{CreateUserRequest, RegisterResponse};
use crate::error::error::AppError;
use crate::models::user::{self, Entity as UserEntity};
use actix_web::{web, HttpResponse, Result};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use log::info;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};
use validator::Validate;
pub async fn login(
    db: web::Data<DatabaseConnection>,
    user_data: web::Json<CreateUserRequest>,
) -> Result<HttpResponse> {
    // TODO: implement login logic
    info!("login user: {}", user_data.user_name);
    Ok(HttpResponse::Ok().json(()))
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

    // 验证用户名不为空
    if user_data.user_name.trim().is_empty() {
        return Err(AppError::BadRequest("用户名不能为空".into()));
    }

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
