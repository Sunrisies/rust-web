use actix_web::{error::ResponseError, HttpResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("请求体解析错误: {0}")]
    DeserializeError(String),

    #[error("Conflict occurred: {0}")]
    Conflict(String), // 添加 Conflict 变体
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Internal(msg) => HttpResponse::InternalServerError().json(ErrorResponse {
                code: 500,
                error: "Internal Server Error".to_string(),
                message: msg.to_string(),
            }),
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: msg.to_string(),
            }),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(ErrorResponse {
                code: 404,
                error: "Not Found".to_string(),
                message: msg.to_string(),
            }),
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().json(ErrorResponse {
                code: 401,
                error: "Unauthorized".to_string(),
                message: msg.to_string(),
            }),
            AppError::DeserializeError(msg) => HttpResponse::BadRequest().json(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: msg.to_string(),
            }),
            AppError::Conflict(msg) => {
                HttpResponse::Conflict().json(serde_json::json!({ "error": msg }))
            }
        }
    }
}

// 添加从 JsonPayloadError 的转换
impl From<actix_web::error::JsonPayloadError> for AppError {
    fn from(err: actix_web::error::JsonPayloadError) -> Self {
        println!("打印数据{:?}", err);
        AppError::DeserializeError(err.to_string())
    }
}
// // 从数据库错误转换
// impl From<sea_orm::DbErr> for AppError {
//     fn from(err: sea_orm::DbErr) -> Self {
//         AppError::Internal(format!("Database error: {}", err))
//     }
// }
