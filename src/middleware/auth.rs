use crate::models::user::{self, Entity as UserEntity};
use crate::AppError;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::HeaderValue,
    Error,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
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
            Box::pin(async move {
                let err = AppError::Unauthorized("权限不够，请申请权限".to_string());
                Err(err.into())
            })
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_id: i32,
    pub user_name: String,
    pub exp: usize, // 令牌过期时间
}

fn has_permission(req: &ServiceRequest) -> bool {
    // 实现你的鉴权逻辑，根据需求判断是否有权限
    // 返回 true 表示有权限，返回 false 表示没有权限
    // unimplemented!()
    let value = HeaderValue::from_str("").unwrap();
    let token = req.headers().get("token").unwrap_or(&value);
    let ls = req.headers().get("Authorization").unwrap_or(&value);
    info!("ls: {}", ls.to_str().unwrap());
    let token_message = decode::<TokenClaims>(
        token.to_str().unwrap(),
        &DecodingKey::from_secret("secret_key".as_bytes()),
        &Validation::new(Algorithm::HS256),
    );
    match token_message {
        Ok(token_data) => {
            let user_id = token_data.claims.user_id;
            let user_name = token_data.claims.user_name;

            // 使用 user_id 和 user_name
            info!("User ID: {}", user_id);
            info!("User Name: {}", user_name);

            // 示例：从数据库中获取用户信息
            // let user = UserEntity::find_by_id(user_id)
            //     .one(db.as_ref())
            //     .await
            //     .map_err(|e| AppError::Internal(format!("获取用户时发生错误: {}", e)))?;

            // 其他逻辑...
        }
        Err(err) => {
            // 处理解码错误
            error!("解码令牌时发生错误: {:?}", err);
            // return Err(AppError::Unauthorized("无效的令牌".into()));
        }
    }

    // 获取完token进行解析
    info!("token: {}", token.to_str().unwrap());
    token.len() > 0 || req.path().to_string() == "/login"
}
