use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::AppError;
use utoipa::ToSchema;
#[derive(Deserialize, Serialize, ToSchema)]
pub struct Resp<T>
where
    T: Serialize,
{
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T: Serialize> Resp<T> {
    pub fn ok(data: T, message: &str) -> Self {
        Resp {
            code: 200,
            message: message.to_owned(),
            data: Some(data),
        }
    }

    pub fn to_json_result(&self) -> Result<HttpResponse, AppError> {
        Ok(HttpResponse::Ok().json(self))
    }
}

impl Resp<()> {
    pub fn err(error: AppError) -> Self {
        match error {
            AppError::BadRequest(msg) => Resp {
                code: 400,
                message: msg,
                data: None,
            },
            AppError::NotFound(msg) => Resp {
                code: 404,
                message: msg,
                data: None,
            },
            AppError::Unauthorized(msg) => Resp {
                code: 401,
                message: msg,
                data: None,
            },
            AppError::DeserializeError(msg) => Resp {
                code: 400,
                message: msg,
                data: None,
            },
            AppError::Conflict(msg) => Resp {
                code: 409,
                message: msg,
                data: None,
            },

            AppError::Forbidden(msg) => Resp {
                code: 403,
                message: msg,
                data: None,
            },

            AppError::InternalServerError(msg) => Resp {
                code: 500,
                message: msg,
                data: None,
            },
            AppError::InvalidTokenFormat => Resp {
                code: 401,
                message: "令牌格式不正确".to_owned(),
                data: None,
            },
            AppError::TokenNotFound => Resp {
                code: 401,
                message: "令牌未找到".to_owned(),
                data: None,
            },
            AppError::PermissionsEmpty => Resp {
                code: 401,
                message: "权限字符串为空".to_owned(),
                data: None,
            },
            AppError::DatabaseError(msg) => Resp {
                code: 500,
                message: msg,
                data: None,
            },
        }
    }
}

pub type SimpleResp = Result<HttpResponse, AppError>;
