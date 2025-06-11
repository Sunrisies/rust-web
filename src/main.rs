use actix_cors::Cors;
use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::http::KeepAlive;
use actix_web::{get, web, HttpResponse};
use actix_web::{middleware::ErrorHandlers, App, HttpServer};
use actix_web_lab::sse::{self, Sse};
use log::info;
use mysql_user_crud::{
    config_routes, create_db_pool, log::init_logger, middleware::auth::Auth,
    utils::error_handler::add_error_header, utils::sse::SseNotifier, AppError, Logger,
};
use std::env;
use std::time::Duration;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
#[get("/sse/{user_id}")]
pub async fn user_sse(
    user_id: web::Path<String>,
    notifier: web::Data<SseNotifier>,
) -> HttpResponse {
    // 创建该用户的SSE通道
    let rx = notifier.create_channel(&user_id);

    // 将 broadcast::Receiver 转换为 Stream
    let stream = BroadcastStream::new(rx);

    // 映射为 SSE 事件流
    let sse_stream = stream
        .filter_map(|msg| {
            match msg {
                Ok(data) => Some(Ok(sse::Event::Data(sse::Data::new(data)))),
                Err(_) => {
                    // 处理广播错误（可选）
                    None
                }
            }
        })
        .map(|res| res.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));

    // 创建 SSE 响应
    // let sse = Sse::from_stream(sse_stream);
    let sse = Sse::new(sse_stream); // 使用 Sse::new 而不是 from_stream
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
        .streaming(sse)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    let db_pool = create_db_pool()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // 将数据库连接池添加到应用程序数据
    let app_data = web::Data::new(db_pool);
    let sse_notifier = web::Data::new(SseNotifier::new());
    // 获取服务器地址和端口
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "18080".to_string());
    let server_addr = format!("{}:{}", host, port);
    info!("11Starting server at http://{}", server_addr);

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
                    .error_handler(|err, _req| AppError::from(err).into()),
            )
            .app_data(sse_notifier.clone())
            .service(user_sse) // 注册SSE端点
            .app_data(app_data.clone())
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
