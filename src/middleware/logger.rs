use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use std::time::Instant;

pub struct Logger;

impl<S, B> Transform<S, ServiceRequest> for Logger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>, // 原始服务类型
    S::Future: 'static,
    B: 'static,
{
    type Transform = LoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type Response = ServiceResponse<B>;
    type Error = Error;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggerMiddleware { service }))
    }
}

pub struct LoggerMiddleware<S> {
    service: S,
}
impl<S, B> Service<ServiceRequest> for LoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().to_string();
        let path = req.path().to_string();
        let start_time = Instant::now();
        log::info!("Request: {} {}", method, path);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;

            let duration = start_time.elapsed();
            log::error!("Response: {:#?}", res.headers());
            log::info!(
                "Response: {} | Time: {:.3}ms",
                res.status(),
                duration.as_secs_f64() * 1000.0
            );

            Ok(res)
        })
    }
}
