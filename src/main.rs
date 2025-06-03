use actix_web::{web, App, HttpServer};
use mysql_user_crud::{Database, config_routes};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // 创建数据库连接
    let db = match Database::new() {
        Ok(db) => {
            // 确保表已创建
            if let Err(e) = db.create_table() {
                eprintln!("Failed to create table: {}", e);
                std::process::exit(1);
            }
            db
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // 获取服务器地址和端口
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let server_addr = format!("{}:{}", host, port);

    println!("Starting server at http://{}", server_addr);

    // 启动 HTTP 服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .configure(config_routes)
    })
    .bind(&server_addr)?
    .run()
    .await
}