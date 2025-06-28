use super::json_error::parse_json_error;
use actix_web::{error::ResponseError, HttpResponse};
use serde::Serialize;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum AppError {
    // 状态码400
    #[error("缺少参数: {0}")]
    BadRequest(String),

    // 状态码404
    #[error("资源未找到: {0}")]
    NotFound(String),

    // 状态码401
    #[error("权限不足: {0}")]
    Unauthorized(String),

    // 状态码400
    #[error("请求体解析错误: {0}")]
    DeserializeError(String),
    // 状态码409
    #[error("数据存在: {0}")]
    Conflict(String),
    // 状态码403
    #[error("禁止访问: {0}")]
    Forbidden(String),
    // 状态码500
    #[error("服务器错误: {0}")]
    InternalServerError(String),

    // 令牌格式不正确错误
    // 状态码 400
    #[error("令牌格式不正确")]
    InvalidTokenFormat,

    // 令牌未找到错误 状态码401
    #[error("令牌未找到")]
    TokenNotFound,

    // 权限字符串为空错误 状态码403
    #[error("权限字符串为空")]
    PermissionsEmpty,

    // 数据库错误 状态码500
    #[error("数据库错误: {0}")]
    DatabaseError(String),
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
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(ErrorResponse {
                code: 400,
                error: "缺少参数".to_string(),
                message: msg.to_string(),
            }),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(ErrorResponse {
                code: 404,
                error: "资源未找到".to_string(),
                message: msg.to_string(),
            }),
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().json(ErrorResponse {
                code: 401,
                error: "权限不足".to_string(),
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

            AppError::Forbidden(msg) => HttpResponse::Forbidden().json(ErrorResponse {
                code: 403,
                error: "禁止访问".to_string(),
                message: msg.to_string(),
            }),

            AppError::InternalServerError(msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    code: 500,
                    error: "服务器错误".to_string(),
                    message: msg.to_string(),
                })
            }
            AppError::InvalidTokenFormat => HttpResponse::BadRequest().json(ErrorResponse {
                code: 400,
                error: "令牌格式不正确".to_string(),
                message: "令牌格式不正确".to_string(),
            }),
            AppError::TokenNotFound => HttpResponse::Unauthorized().json(ErrorResponse {
                code: 401,
                error: "令牌未找到".to_string(),
                message: "令牌未找到".to_string(),
            }),
            AppError::PermissionsEmpty => HttpResponse::Forbidden().json(ErrorResponse {
                code: 403,
                error: "权限字符串为空".to_string(),
                message: "权限字符串为空".to_string(),
            }),
            AppError::DatabaseError(msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    code: 500,
                    error: "数据库错误".to_string(),
                    message: msg.to_string(),
                })
            }
        }
    }
}

// 添加从 JsonPayloadError 的转换
impl From<actix_web::error::JsonPayloadError> for AppError {
    fn from(err: actix_web::error::JsonPayloadError) -> Self {
        log::error!("JSON解析错误: {}", err);
        log::error!("JSON解析错误: {}", err.to_string());
        let message = parse_json_error(&err);
        Self::DeserializeError(message)
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(error: sea_orm::DbErr) -> Self {
        AppError::DatabaseError(error.to_string())
    }
}
