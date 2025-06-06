use crate::json_error::parse_json_error;
use actix_web::{error::ResponseError, HttpResponse};
use serde::Serialize;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum AppError {
    #[error("服务器错误: {0}")]
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
    Conflict(String),

    // 缺少参数
    #[error("缺少参数: {0}")]
    MissingParameter(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}
// 错误处理
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Internal(msg) => HttpResponse::InternalServerError().json(ErrorResponse {
                code: 500,
                error: "服务器错误".to_string(),
                message: msg.to_string(),
            }),
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(ErrorResponse {
                code: 400,
                error: "缺少参数".to_string(),
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
                error: "参数类型错误".to_string(),
                message: msg.to_string(),
            }),
            AppError::Conflict(msg) => {
                HttpResponse::Conflict().json(serde_json::json!({ "error": msg }))
            }
            AppError::MissingParameter(msg) => HttpResponse::BadRequest().json(ErrorResponse {
                code: 400,
                error: "Missing Parameter".to_string(),
                message: msg.to_string(),
            }),
        }
    }
}

// 添加从 JsonPayloadError 的转换
impl From<actix_web::error::JsonPayloadError> for AppError {
    fn from(err: actix_web::error::JsonPayloadError) -> Self {
        let message = parse_json_error(&err);
        Self::BadRequest(message)
    }
}
