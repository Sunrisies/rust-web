use actix_web::{
    dev::ServiceResponse,
    http::StatusCode,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, App, HttpMessage, HttpResponse, HttpServer, Result,
};
use log::{error, info};
use mysql_user_crud::{
    config_routes, create_db_pool, log::init_logger, middleware::auth::Auth, AppError, Logger,
};
use serde_json::json;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

fn add_error_header<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let (req, res) = res.into_parts();

    let error_response = match req.extensions().get::<Rc<RefCell<Option<AppError>>>>() {
        Some(error_cell) => {
            error!("error_cel1111l: {:?}", error_cell);
            let error = error_cell.borrow();
            if let Some(app_error) = error.as_ref() {
                // 从AppError中提取错误信息
                let error_message = app_error.to_string();
                error!("拦截到应用错误: {}", error_message);

                // 根据错误类型设置HTTP状态码
                let status_code = match app_error {
                    AppError::Forbidden(_) => StatusCode::FORBIDDEN,
                    AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                // 构建JSON响应
                HttpResponse::build(status_code).json(json!({
                    "message": error_message,
                    "code": status_code.as_u16()
                }))
            } else {
                // 没有具体错误时的默认响应
                HttpResponse::InternalServerError().json(json!({
                    "message": "未知错误",
                    "code": 500
                }))
            }
        }
        None => {
            error!("请求中未找到错误信息");
            HttpResponse::InternalServerError().json(json!({
                "message": "服务器内部错误",
                "code": 500
            }))
        }
    };

    let new_res = ServiceResponse::new(req, error_response)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(new_res))
}

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
            .app_data(
                web::JsonConfig::default()
                    .limit(4096) // 限制请求体大小
                    .error_handler(|err, _req| AppError::from(err).into()),
            )
            .app_data(app_data.clone())
            .wrap(Logger)
            .wrap(Auth)
            .wrap(actix_web::middleware::Logger::default())
            .wrap(ErrorHandlers::new().default_handler(add_error_header))
            .configure(config_routes)
    })
    .bind(&server_addr)?
    .run()
    .await
}
