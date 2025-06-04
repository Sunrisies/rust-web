use actix_web::error::JsonPayloadError;
use actix_web::{error::ResponseError, HttpResponse};
use anyhow::Ok;
// pub use json_error::parse_json_error;
use regex::Regex;
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
/// 从 JSON 反序列化错误中提取友好信息
pub fn parse_json_error(err: &JsonPayloadError) -> String {
    let error_str = err.to_string();

    // 使用正则表达式提取关键信息
    let re = Regex::new(r"missing field `([^`]+)`").unwrap();
    if let Some(caps) = re.captures(&error_str) {
        if let Some(field) = caps.get(1) {
            return format!("缺少必填字段: {}", field.as_str());
        }
    }

    // 处理其他常见错误类型
    if error_str.contains("expected") && error_str.contains("found") {
        return "字段类型不匹配".to_string();
    }

    if error_str.contains("unexpected end of input") {
        return "请求体不完整".to_string();
    }

    // 默认错误信息
    "请求数据格式错误".to_string()
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
