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

    #[error("权限不足: {0}")]
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
                code: 403,
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

// #[derive(Debug, Serialize)]
// pub struct CustomError {
//     pub code: u16,
//     pub message: String,
// }

// impl From<actix_web::Error> for CustomError {
//     fn from(e: actix_web::Error) -> Self {
//         CustomError {
//             code: e.as_response_error().status_code().as_u16(),
//             message: e.to_string(),
//         }
//     }
// }

// impl ResponseError for CustomError {
//     fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
//         HttpResponse::build(self.status_code()).json(self)
//     }

//     fn status_code(&self) -> actix_web::http::StatusCode {
//         actix_web::http::StatusCode::from_u16(self.code)
//             .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
//     }
// }

// impl From<actix_web::Error> for CustomError {
//     fn from(e: actix_web::Error) -> Self {
//         CustomError {
//             code: e.as_response_error().status_code().as_u16(),
//             message: e.to_string(),
//         }
//     }
// }
