use actix_web::{web, App, HttpServer};
use log::info;
use mysql_user_crud::{
    config_routes, create_db_pool, log::init_logger, middleware::auth::Auth, AppError, Logger,
};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    let db_pool = create_db_pool()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // 将数据库连接池添加到应用程序数据
    let app_data = web::Data::new(db_pool);

    // 获取服务器地址和端口
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "18080".to_string());
    let server_addr = format!("{}:{}", host, port);
    info!("11Starting server at http://{}", server_addr);

    // 启动 HTTP 服务器
    HttpServer::new(move || {
        App::new()
            // .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header))
            .app_data(
                web::JsonConfig::default()
                    .limit(4096) // 限制请求体大小
                    .error_handler(|err, _req| AppError::from(err).into()),
            )
            .app_data(app_data.clone())
            .wrap(Logger)
            .wrap(Auth)
            .wrap(actix_web::middleware::Logger::default())
            .configure(config_routes)
    })
    .bind(&server_addr)?
    .run()
    .await
}
