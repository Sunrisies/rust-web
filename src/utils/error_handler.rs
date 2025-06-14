use crate::AppError;
use actix_web::{
    body::MessageBody, dev::ServiceResponse, http::StatusCode, middleware::ErrorHandlerResponse,
    HttpMessage, HttpResponse, Result,
};
use log::error;
use serde_json::json;
use std::cell::RefCell;
use std::rc::Rc;

pub fn add_error_header<B: MessageBody + 'static>(
    res: ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    let (req, _res) = res.into_parts();

    if let Some(app_err) = req.extensions().get::<Rc<RefCell<Option<AppError>>>>() {
        let app_err = app_err.borrow();
        if let Some(app_error) = app_err.as_ref() {
            let error_message = app_error.to_string();
            error!("拦截到应用错误: {}", error_message);

            let status_code = match app_error {
                AppError::Forbidden(_) => StatusCode::FORBIDDEN,
                AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
                AppError::NotFound(_) => StatusCode::NOT_FOUND,
                AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
                AppError::DeserializeError(_) => StatusCode::BAD_REQUEST,
                AppError::Conflict(_) => StatusCode::CONFLICT,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            let error_response = HttpResponse::build(status_code).json(json!({
                "message": error_message,
                "code": status_code.as_u16()
            }));

            let new_res = ServiceResponse::new(req.clone(), error_response)
                .map_into_boxed_body()
                .map_into_right_body();

            return Ok(ErrorHandlerResponse::Response(new_res));
        }
    }

    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(req, _res)
            .map_into_boxed_body()
            .map_into_right_body(),
    ))
}
