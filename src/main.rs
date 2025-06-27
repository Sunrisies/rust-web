use actix_cors::Cors;
use actix_web::{middleware::ErrorHandlers, web, App, HttpServer};
use mysql_user_crud::{
    config_routes, create_db_pool, log::init_logger, middleware::auth::Auth,
    utils::error_handler::add_error_header, utils::sse::SseNotifier, AppError, Logger,
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
    let notifier = web::Data::new(SseNotifier::new());
    // 获取服务器地址和端口
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "18080".to_string());
    let server_addr = format!("{}:{}", host, port);
    log::info!("当前服务成功启动，监听地址为 http://{}", server_addr);

    // 启动 HTTP 服务器
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:5502")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization", "ACCEPT"])
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(ErrorHandlers::new().default_handler(add_error_header))
            .app_data(
                web::JsonConfig::default()
                    .limit(4096) // 限制请求体大小
                    .error_handler(|err, _req| {
                        log::error!("JSON 解析错误: {}0-----", err);
                        AppError::from(err).into()
                    }),
            )
            .app_data(notifier.clone())
            .app_data(app_data.clone())
            // .wrap(ResponseMiddleware)
            .wrap(Logger)
            .wrap(Auth)
            .wrap(actix_web::middleware::Logger::default())
            .configure(config_routes)
            .wrap(cors)
    })
    .bind(&server_addr)?
    .run()
    .await
}
