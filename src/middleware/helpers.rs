use crate::jsonwebtoken::{extract_token, has_permission};
use crate::AppError;

use actix_web::body;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
// use futures_util::future::LocalBoxFuture;
use actix_web::body::MessageBody;
use log::{error, info};
use sea_orm::DatabaseConnection;
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

pub type DbPool = DatabaseConnection;
pub struct ResponseMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ResponseMiddleware
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
type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;
impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        // A more complex middleware, could return an error or an early response here.

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let body = res.request();
            // let sls = res.response().into_body().into();
            // log::error!("{:?}", sls);
            // println!("Hi from response{:?}", sls);
            println!("Hi from response{:?}", body);
            Ok(res)
        })
    }
}
