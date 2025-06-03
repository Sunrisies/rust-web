use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

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

// 获取所有用户
pub async fn get_all_users(db: web::Data<Database>) -> Result<HttpResponse> {
    match db.get_all_users() {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
}

// 创建新用户
pub async fn create_user(
    db: web::Data<Database>,
    user_data: web::Json<CreateUserRequest>,
) -> Result<HttpResponse> {
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
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
}

// 通过ID获取用户
pub async fn get_user_by_id(
    db: web::Data<Database>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    match db.get_user_by_id(*id) {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Ok(HttpResponse::NotFound().body("User not found")),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
}

// 更新用户
pub async fn update_user(
    db: web::Data<Database>,
    id: web::Path<i32>,
    user_data: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse> {
    let user = User {
        id: Some(*id),
        username: user_data.username.clone(),
        email: user_data.email.clone(),
        age: user_data.age,
    };

    match db.update_user(&user) {
        Ok(_) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
}

// 删除用户
pub async fn delete_user(
    db: web::Data<Database>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    match db.delete_user(*id) {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
}