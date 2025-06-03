use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct User {
    id: i32,
    username: String,
    email: String,
    age: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct NewUser {
    username: String,
    email: String,
    age: i32,
}

struct AppState {
    pool: Pool,
}

impl AppState {
    fn new() -> Result<Self> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let opts = Opts::from_url(&database_url).expect("Invalid DATABASE_URL");
        let pool = Pool::new(opts)?;
        Ok(AppState { pool })
    }

    // 获取所有用户
    async fn get_all_users(&self) -> Result<Vec<User>> {
        let mut conn = self.pool.get_conn()?;
        let users = conn.query_map(
            "SELECT id, username, email, age FROM users",
            |(id, username, email, age)| User {
                id,
                username,
                email,
                age,
            },
        )?;
        Ok(users)
    }

    // 通过ID获取用户
    async fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>> {
        let mut conn = self.pool.get_conn()?;
        let user = conn.exec_first(
            "SELECT id, username, email, age FROM users WHERE id = :id",
            params! { "id" => user_id },
        )?;
        Ok(user)
    }

    // 创建用户
    async fn create_user(&self, new_user: &NewUser) -> Result<()> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "INSERT INTO users (username, email, age) VALUES (:username, :email, :age)",
            params! {
                "username" => &new_user.username,
                "email" => &new_user.email,
                "age" => new_user.age,
            },
        )?;
        Ok(())
    }

    // 更新用户
    async fn update_user(&self, user_id: i32, update_user: &NewUser) -> Result<()> {
        println!("更新用户 id: {}", user_id);
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "UPDATE users SET username = :username, email = :email, age = :age WHERE id = :id",
            params! {
                "username" => &update_user.username,
                "email" => &update_user.email,
                "age" => update_user.age,
                "id" => user_id,
            },
        )?;
        Ok(())
    }

    // 删除用户
    async fn delete_user(&self, user_id: i32) -> Result<()> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "DELETE FROM users WHERE id = :id",
            params! { "id" => user_id },
        )?;
        Ok(())
    }
}

// RESTful API处理函数

// 获取所有用户
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    match data.get_all_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// 获取单个用户
async fn get_user(data: web::Data<AppState>, user_id: web::Path<i32>) -> impl Responder {
    match data.get_user_by_id(*user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// 创建用户
async fn create_user(data: web::Data<AppState>, new_user: web::Json<NewUser>) -> impl Responder {
    match data.create_user(&new_user.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// 更新用户
async fn update_user(
    data: web::Data<AppState>,
    user_id: web::Path<i32>,
    update_user: web::Json<NewUser>,
) -> impl Responder {
    match data
        .update_user(*user_id, &update_user.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// 删除用户
async fn delete_user(data: web::Data<AppState>, user_id: web::Path<i32>) -> impl Responder {
    match data.delete_user(*user_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化数据库连接池
    let app_state = match AppState::new() {
        Ok(state) => web::Data::new(state),
        Err(e) => {
            eprintln!("Failed to create database pool: {}", e);
            std::process::exit(1);
        }
    };

    println!("Server running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                web::scope("/api/users")
                    .route("", web::get().to(get_users))
                    .route("", web::post().to(create_user))
                    .route("/{id}", web::get().to(get_user))
                    .route("/{id}", web::put().to(update_user))
                    .route("/{id}", web::delete().to(delete_user)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
