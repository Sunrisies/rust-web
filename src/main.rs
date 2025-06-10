use actix_web::{
    dev::ServiceResponse,
    error,
    http::{header, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, App, HttpMessage, HttpResponse, HttpServer, Responder, Result,
};
use log::{error, info};
use mysql_user_crud::{
    config_routes, create_db_pool, log::init_logger, middleware::auth::Auth, AppError, Logger,
};
use serde_json::json;
use std::cell::RefCell;
use std::env;
use std::future;

use std::rc::Rc;
// 自定义错误容器
#[derive(Debug)]
pub struct GuardError {
    pub status: StatusCode,
    pub message: String,
}

fn add_error_header<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let (req, mut res) = res.into_parts();
    match req
        .extensions_mut()
        .get_mut::<Rc<RefCell<Option<GuardError>>>>()
    {
        Some(error_cell) => {
            error!("error_cell: {:?}", error_cell);
        }
        None => {
            error!("No error cell found in request extensions");
        }
    }
    match res
        .extensions_mut()
        .get_mut::<Rc<RefCell<Option<GuardError>>>>()
    {
        Some(error_cell) => {
            error!("error_cell: {:?}", error_cell);
        }
        None => {
            error!("No error cell found in request extensions");
        }
    }
    match req.extensions().get::<Rc<RefCell<Option<GuardError>>>>() {
        Some(error_cell) => {
            error!("error_cell: {:?}", error_cell);
        }
        None => {
            error!("No error cell found in request extensions1111");
        }
    }
    match res.extensions().get::<Rc<RefCell<Option<GuardError>>>>() {
        Some(error_cell) => {
            error!("error_cell: {:?}", error_cell);
        }
        None => {
            error!("No error cell found in request extensions1111");
        }
    }
    // // set body of response to modified body
    if let Some(error_cell) = req
        .extensions_mut()
        .get_mut::<Rc<RefCell<Option<GuardError>>>>()
    {
        error!("error_cell: {:?}", error_cell);
    }
    let res1 = res.set_body("An error occurred.");

    let res11 = ServiceResponse::new(req, res1)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res11))
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
