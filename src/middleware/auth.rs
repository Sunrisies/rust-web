use crate::jsonwebtoken::has_permission;
use crate::AppError;
use actix_web::http::header::HeaderMap;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use log::{error, info};
use sea_orm::DatabaseConnection;
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

pub type DbPool = DatabaseConnection;
pub struct Auth;

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
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string(); // 克隆 path
        let headers = req.headers().clone(); // 克隆 headers
        error!("path: {}, headers: {:?}", path, headers);
        let fut = self.service.call(req);
        Box::pin(async move {
            let public_paths = vec![
                "/api/auth/login",
                "/api/auth/register",
                "/api/posts",
                "/api/comments",
            ];
            if public_paths.contains(&path.as_str()) {
                let res = fut.await?;
                Ok(res)
            } else {
                let token = extract_token(&headers).await;
                if let Some(token) = token {
                    let permission_result = has_permission(&token);
                    match permission_result {
                        Ok(_token_data) => {
                            info!("令牌有效");
                            let res = fut.await?;
                            Ok(res)
                        }
                        Err(err) => {
                            // 处理解码错误
                            error!("解码令牌时发生错误: {:?}", err);
                            let err = AppError::Unauthorized("无效的令牌".to_string());
                            Err(err.into())
                        }
                    }
                } else {
                    // 没有找到令牌
                    let err = AppError::Unauthorized("令牌未找到".to_string());
                    Err(err.into())
                }
            }
        })
    }
}

async fn extract_token(headers: &HeaderMap) -> Option<String> {
    if let Some(authorization_header) = headers.get("Authorization") {
        if let Ok(authorization_str) = authorization_header.to_str() {
            // 假设令牌格式为 "Bearer <token>"
            if let Some(token) = authorization_str.strip_prefix("Bearer ") {
                return Some(token.to_string());
            }
        }
    }
    None
}
