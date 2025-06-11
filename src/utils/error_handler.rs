// use actix_web::{
//     dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
//     Error,
// };
// use futures_util::future::{LocalBoxFuture, Ready};
// use std::rc::Rc;

// // // 中间件工厂
// // pub struct ErrorHandler;

// // impl<S: 'static, B> Transform<S, ServiceResponse> for ErrorHandler
// // where
// //     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
// //     B: 'static,
// // {
// //     type Response = ServiceResponse<B>;
// //     type Error = Error;
// //     type Transform = ErrorHandlerMiddleware<S>;
// //     type InitError = ();
// //     type Future = Ready<Result<Self::Transform, Self::InitError>>;

// //     // fn new_transform(&self, service: S) -> Self::Future {
// //     //     future::ready(Ok(ErrorHandlerMiddleware {
// //     //         service: Rc::new(service),
// //     //     }))
// //     // }
// // }

// // // 中间件实现
// // pub struct ErrorHandlerMiddleware<S> {
// //     service: S,
// // }

// // impl<S, B> Service<ServiceRequest> for ErrorHandlerMiddleware<S>
// // where
// //     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
// //     B: 'static,
// // {
// //     type Response = ServiceResponse<B>;
// //     type Error = Error;
// //     type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

// //     actix_web::dev::forward_ready!(service);

// //     fn call(&self, req: ServiceRequest) -> Self::Future {
// //         let service = Rc::clone(&self.service);
// //         Box::pin(async move {
// //             // 先执行后续服务（控制器等）
// //             let res = service.call(req).await?;

// //             // 检查响应状态码
// //             if res.status().is_server_error() {
// //                 // 获取原始响应
// //                 let (req, res) = res.into_parts();
// //                 let (res, body) = res.into_parts();

// //                 // 构建自定义错误响应
// //                 let error_json = serde_json::json!({
// //                     "error": "Internal Server Error",
// //                     "message": "Something went wrong on our side",
// //                     "status": res.status().as_u16(),
// //                 });

// //                 let new_res = HttpResponse::build(res.status())
// //                     .content_type("application/json")
// //                     .body(serde_json::to_string(&error_json).unwrap());

// //                 // 返回新响应
// //                 Ok(ServiceResponse::new(req, new_res))
// //             } else {
// //                 // 非错误响应直接返回
// //                 Ok(res)
// //             }
// //         })
// //     }
// // }

// // 定义日志中间件结构体
// pub struct LogMiddleware;

// // 实现 Transform trait for LogMiddleware
// impl<S, B> Transform<S, ServiceRequest> for LogMiddleware
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type InitError = ();
//     type Transform = LogService<S>;
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         futures::future::ready(Ok(LogService {
//             service,
//             logger: Rc::new(|| ()),
//         }))
//     }
// }

// // 定义日志服务结构体，包装原始服务
// pub struct LogService<S> {
//     service: S,
//     logger: Rc<dyn Fn()>,
// }

// // 实现 Service trait for LogService
// impl<S, B> Service<ServiceRequest> for LogService<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

//     forward_ready!(service);

//     fn call(&self, req: ServiceRequest) -> Self::Future {
//         let logger = Rc::clone(&self.logger);
//         let fut = self.service.call(req);

//         Box::pin(async move {
//             // 在请求处理前记录日志
//             // println!("Request: {:?}", req.path());
//             // (logger)();
//             let res = fut.await?;
//             // 在响应生成后记录日志
//             println!("Response: {:?}", res.status());
//             Ok(res)
//         })
//     }
// }
