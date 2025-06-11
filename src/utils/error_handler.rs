use crate::AppError;
use actix_web::{
    dev::ServiceResponse, http::StatusCode, middleware::ErrorHandlerResponse, HttpMessage,
    HttpResponse, Result,
};
use log::error;
use serde_json::json;
use std::cell::RefCell;
use std::rc::Rc;

pub fn add_error_header<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let (req, _res) = res.into_parts();

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
