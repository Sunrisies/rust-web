use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::db::Database;
use crate::models::User;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    username: String,
    email: String,
    age: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    username: String,
    email: String,
    age: Option<i32>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// 获取所有用户
pub async fn get_all_users(db: web::Data<Database>) -> Result<HttpResponse> {
    match db.get_all_users() {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => {
            let error_response = ErrorResponse {
                error: e.to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}

// 创建新用户
pub async fn create_user(
    db: web::Data<Database>,
    user_data: web::Json<CreateUserRequest>,
) -> Result<HttpResponse> {
    // 检查用户名是否已存在
    match db.username_exists(&user_data.username) {
        Ok(exists) => {
            if exists {
                let error_response = ErrorResponse {
                    error: format!("用户名 '{}' 已存在", user_data.username),
                };
                return Ok(HttpResponse::BadRequest().json(error_response));
            }
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: format!("检查用户名时发生错误: {}", e),
            };
            return Ok(HttpResponse::InternalServerError().json(error_response));
        }
    }

    let user = User::new(
        user_data.username.clone(),
        user_data.email.clone(),
        user_data.age,
    );

    match db.create_user(&user) {
        Ok(id) => {
            let mut created_user = user;
            created_user.id = Some(id as i32);
            Ok(HttpResponse::Created().json(created_user))
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: e.to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}

// 通过ID获取用户
pub async fn get_user_by_id(
    db: web::Data<Database>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    match db.get_user_by_id(*id) {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => {
            let error_response = ErrorResponse {
                error: "用户未找到".to_string(),
            };
            Ok(HttpResponse::NotFound().json(error_response))
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: e.to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}

// 更新用户
pub async fn update_user(
    db: web::Data<Database>,
    id: web::Path<i32>,
    user_data: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse> {
    // 检查新用户名是否与其他用户冲突
    match db.get_user_by_id(*id) {
        Ok(Some(existing_user)) => {
            if existing_user.username != user_data.username {
                // 只有当用户名发生变化时才检查
                if let Ok(exists) = db.username_exists(&user_data.username) {
                    if exists {
                        let error_response = ErrorResponse {
                            error: format!("用户名 '{}' 已存在", user_data.username),
                        };
                        return Ok(HttpResponse::BadRequest().json(error_response));
                    }
                }
            }
        }
        Ok(None) => {
            let error_response = ErrorResponse {
                error: "要更新的用户不存在".to_string(),
            };
            return Ok(HttpResponse::NotFound().json(error_response));
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: e.to_string(),
            };
            return Ok(HttpResponse::InternalServerError().json(error_response));
        }
    }

    let user = User {
        id: Some(*id),
        username: user_data.username.clone(),
        email: user_data.email.clone(),
        age: user_data.age,
    };

    match db.update_user(&user) {
        Ok(_) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            let error_response = ErrorResponse {
                error: e.to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}

// 删除用户
pub async fn delete_user(
    db: web::Data<Database>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    match db.delete_user(*id) {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            let error_response = ErrorResponse {
                error: e.to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}