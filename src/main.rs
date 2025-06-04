use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use mysql_user_crud::config_routes;

use sea_orm::{ConnectOptions, Database};
use std::env;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema
    let db = Database::connect(opt)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

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
