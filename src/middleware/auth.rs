// use crate::config::permission::Permission;
use crate::models::user::{self, Entity as UserEntity};
use crate::AppError;
use actix_web::http::header;
use actix_web::HttpMessage;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::HeaderValue,
    Error,
};
// use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use log::{error, info};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

pub type DbPool = DatabaseConnection;
pub struct Auth {
    db_pool: Arc<DbPool>, // 添加数据库连接池字段
}

impl Auth {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        Self { db_pool }
    }
}

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
        ready(Ok(AuthMiddleware {
            service,
            db_pool: self.db_pool.clone(),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
    db_pool: Arc<DbPool>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    // type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // let req_arc = Arc::new(req);
        let db_pool = Arc::clone(&self.db_pool);
        // 进行鉴权操作，判断是否有权限
        // if has_permission(&req, &db_pool) {
        //     // 有权限，继续执行后续中间件
        //     let fut = self.service.call(req);
        //     Box::pin(async move {
        //         let res = fut.await?;
        //         Ok(res)
        //     })
        // } else {
        //     Box::pin(async move {
        //         let err = AppError::Unauthorized("权限不够，请申请权限".to_string());
        //         Err(err.into())
        //     })
        // }
        // let req_clone = req.clone();
        let path = req.path().to_string(); // 克隆 path
        let headers = req.headers().clone(); // 克隆 headers

        let fut = self.service.call(req);
        Box::pin(async move {
            match has_permission(&headers, &db_pool).await {
                Ok(Some(permission)) => {
                    let res = fut.await?;
                    Ok(res)
                }
                Ok(None) => {
                    // 权限不够，请申请权限
                    let err = AppError::Unauthorized("权限不够，请申请权限".to_string());
                    Err(err.into())
                }
                Err(err) => {
                    // 处理解码错误
                    error!("解码令牌时发生错误: {:?}", err);
                    let err = AppError::Unauthorized("无效的令牌".to_string());
                    Err(err.into())
                }
            }
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_uuid: String,
    pub user_name: String,
    pub exp: usize, // 令牌过期时间
}

// fn check_permission(permission: &Permission, target_permission: Permission) {
//     if permission.contains(target_permission) {
//         println!("具有权限");
//     } else {
//         println!("无权限");
//     }
// }
//   // 实现你的鉴权逻辑，根据需求判断是否有权限
//     // 返回 true 表示有权限，返回 false 表示没有权限
//     // unimplemented!()
//     // 分配权限
//     let admin_permission = Permission::ALL;
//     let editor_permission = Permission::READ_WRITE_ARTICLE | Permission::READ_WRITE_COMMENT;
//     let user_permission = Permission::READ_ARTICLE | Permission::READ_COMMENT;
//     let guest_permission = Permission::NONE;

//     // 权限检查
//     check_permission(&admin_permission, Permission::WRITE_ARTICLE); // 应该输出：具有权限
//     check_permission(&editor_permission, Permission::WRITE_USER); // 应该输出：无权限
//     check_permission(&user_permission, Permission::READ_ARTICLE); // 应该输出：具有权限
//     check_permission(&guest_permission, Permission::READ_SYSTEM); // 应该输出：无权限
async fn has_permission(
    headers: &actix_web::http::header::HeaderMap,
    db_pool: &Arc<DbPool>,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let db: &DatabaseConnection = &db_pool;
    let value = HeaderValue::from_str("").unwrap();
    let token = headers.get("token").unwrap_or(&value);
    let ls = headers.get("Authorization").unwrap_or(&value);
    info!("ls: {}", ls.to_str().unwrap());
    let token_message = decode::<TokenClaims>(
        token.to_str().unwrap(),
        &DecodingKey::from_secret("secret_key".as_bytes()),
        &Validation::new(Algorithm::HS256),
    );
    match token_message {
        Ok(token_data) => {
            let user_uuid = token_data.claims.user_uuid;
            let user_name = token_data.claims.user_name;

            // 使用 user_id 和 user_name
            info!("User ID: {}", user_uuid);
            info!("User Name: {}", user_name);

            match UserEntity::find_by_uuid(&user_uuid).one(db).await {
                Ok(Some(user)) => {
                    info!("User: {:#?}", user);
                    if let Some(permissions) = user.permissions {
                        info!("Permissions from database: {}", permissions);
                        return Ok(Some(permissions));
                    } else {
                        error!("User has no permissions set.");
                        Ok(None)
                    }
                }
                Ok(None) => {
                    error!("User not found.");
                    Ok(None)
                } // 用户不存在
                Err(err) => {
                    error!("Database error: {:?}", err);
                    Err(err.into())
                } // 查询错误
            }
        }
        Err(err) => {
            // 处理解码错误
            error!("解码令牌时发生错误: {:?}", err);
            Err(err.into())
            // return Err(AppError::Unauthorized("无效的令牌".into()));
        }
    }

    // 获取完token进行解析
    // info!("token: {}", token.to_str().unwrap());
    // token.len() > 0 || path.to_string() == "/login"
}
