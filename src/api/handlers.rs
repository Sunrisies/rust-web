use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::db::Database;
use crate::models::User;

const DEFAULT_PAGE_SIZE: i64 = 10;
const MAX_PAGE_SIZE: i64 = 100;

#[derive(Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    page: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    DEFAULT_PAGE_SIZE
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    data: Vec<T>,
    pagination: PaginationInfo,
}

#[derive(Serialize)]
pub struct PaginationInfo {
    total: i64,
    total_pages: i64,
    current_page: i64,
    limit: i64,
    has_next: bool,
    has_previous: bool,
}

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

// 获取用户列表（带分页）
pub async fn get_all_users(
    db: web::Data<Database>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse> {
    // 验证分页参数
    if query.page <= 0 {
        let error_response = ErrorResponse {
            error: "页码必须大于0".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }

    // 限制每页数量的范围
    let limit = if query.limit <= 0 {
        DEFAULT_PAGE_SIZE
    } else if query.limit > MAX_PAGE_SIZE {
        MAX_PAGE_SIZE
    } else {
        query.limit
    };

    match db.get_users_paginated(query.page, limit) {
        Ok(paginated) => {
            let response = PaginatedResponse {
                data: paginated.users,
                pagination: PaginationInfo {
                    total: paginated.total,
                    total_pages: paginated.total_pages,
                    current_page: paginated.current_page,
                    limit: paginated.limit,
                    has_next: paginated.current_page < paginated.total_pages,
                    has_previous: paginated.current_page > 1,
                },
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: format!("获取用户列表失败: {}", e),
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
    // 验证用户名不为空
    if user_data.username.trim().is_empty() {
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
                error: format!("创建用户失败: {}", e),
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
    if *id <= 0 {
        let error_response = ErrorResponse {
            error: "用户ID必须大于0".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }

    match db.get_user_by_id(*id) {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => {
            let error_response = ErrorResponse {
                error: format!("ID为{}的用户不存在", id),
            };
            Ok(HttpResponse::NotFound().json(error_response))
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: format!("获取用户信息失败: {}", e),
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
    // 验证ID
    if *id <= 0 {
        let error_response = ErrorResponse {
            error: "用户ID必须大于0".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }

    // 验证用户名不为空
    if user_data.username.trim().is_empty() {
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

    // 检查用户是否存在并验证用户名
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
                error: format!("ID为{}的用户不存在", id),
            };
            return Ok(HttpResponse::NotFound().json(error_response));
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: format!("检查用户信息失败: {}", e),
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
                error: format!("更新用户信息失败: {}", e),
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
    // 验证ID
    if *id <= 0 {
        let error_response = ErrorResponse {
            error: "用户ID必须大于0".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(error_response));
    }

    // 检查用户是否存在
    match db.get_user_by_id(*id) {
        Ok(Some(_)) => {
            match db.delete_user(*id) {
                Ok(_) => Ok(HttpResponse::NoContent().finish()),
                Err(e) => {
                    let error_response = ErrorResponse {
                        error: format!("删除用户失败: {}", e),
                    };
                    Ok(HttpResponse::InternalServerError().json(error_response))
                }
            }
        }
        Ok(None) => {
            let error_response = ErrorResponse {
                error: format!("ID为{}的用户不存在", id),
            };
            Ok(HttpResponse::NotFound().json(error_response))
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: format!("检查用户信息失败: {}", e),
            };
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}