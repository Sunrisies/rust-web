use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use std::time::Instant;

// 日志中间件结构体（空结构体，仅作为标记）
pub struct Logger;

// 实现Transform trait，将普通服务转换为带日志功能的中间件
impl<S, B> Transform<S, ServiceRequest> for Logger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>, // 原始服务类型
    S::Future: 'static, // 服务返回的future必须是静态生命周期
    B: 'static,         // 响应体类型必须是静态生命周期
{
    // 定义转换后的中间件类型
    type Transform = LoggerMiddleware<S>;
    // 初始化错误类型（无错误）
    type InitError = ();
    // 转换操作返回的future类型
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    // 中间件处理后的响应类型
    type Response = ServiceResponse<B>;
    // 中间件错误类型
    type Error = Error;

    // 创建中间件实例的核心方法
    fn new_transform(&self, service: S) -> Self::Future {
        // 立即返回包装后的中间件
        ready(Ok(LoggerMiddleware { service }))
    }
}

// 实际执行日志功能的中间件包装器
pub struct LoggerMiddleware<S> {
    service: S, // 被包装的原始服务
}
// 为中间件实现Service trait
impl<S, B> Service<ServiceRequest> for LoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    // 定义中间件处理后的响应类型
    type Response = ServiceResponse<B>;
    // 中间件错误类型
    type Error = Error;
    // 异步处理返回的future类型
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // 检查服务是否就绪（直接透传给原始服务）
    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    // 实际处理请求的方法
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 记录请求信息（方法+路径）
        let method = req.method().to_string();
        let path = req.path().to_string();
        let start_time = Instant::now();
        log::info!("Request: {} {}", method, path);

        // 创建异步块处理响应
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;

            //    计算请求处理耗时
            let duration = start_time.elapsed();
            // 打印一下res的所有数据
            // log::debug!("Response: {:#?}", res);
            log::error!("Response: {:#?}", res.headers());
            // 记录响应状态码和处理时间
            log::info!(
                "Response: {} | Time: {:.3}ms",
                res.status(),
                duration.as_secs_f64() * 1000.0
            );

            // 返回最终响应
            Ok(res)
        })
    }
}
