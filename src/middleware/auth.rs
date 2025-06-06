use actix_web::web;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error,
    http::header::HeaderValue,
    middleware::ErrorHandlerResponse,
    Error, HttpResponse,
};
use futures_util::{
    future::{self, LocalBoxFuture},
    FutureExt,
};
use log::info;
use serde_json::json;
use std::future::{ready, Ready};

use crate::AppError;
// 定义错误响应结构体
#[derive(Debug, serde::Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}
// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 进行鉴权操作，判断是否有权限
        if has_permission(&req) {
            // 有权限，继续执行后续中间件
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            // 没有权限，立即返回响应
            Box::pin(async move {
                Err(error::ErrorUnauthorized(AppError::Unauthorized(
                    "权限不够".to_string(),
                )))
            })
        }
    }
}

fn has_permission(req: &ServiceRequest) -> bool {
    // 实现你的鉴权逻辑，根据需求判断是否有权限
    // 返回 true 表示有权限，返回 false 表示没有权限
    // unimplemented!()
    let value = HeaderValue::from_str("").unwrap();
    let token = req.headers().get("token").unwrap_or(&value);
    info!("token: {}", token.to_str().unwrap());
    token.len() > 0 || req.path().to_string() == "/login"
}
