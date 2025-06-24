// // use crate::AppError;
// // use actix_web::guard::{Guard, GuardContext};
// // use log::{error, info};
// // use std::cell::RefCell;
// // use std::rc::Rc;

// // pub struct PaginationGuard;

// // impl Guard for PaginationGuard {
// //     fn check(&self, ctx: &GuardContext<'_>) -> bool {
// //         match self.check_permission(ctx) {
// //             Ok(result) => result,
// //             Err(err) => {
// //                 ctx.req_data_mut().insert(Rc::new(RefCell::new(Some(err))));
// //                 false
// //             }
// //         }
// //     }
// // }

// // impl PaginationGuard {
// //     fn check_permission(&self, ctx: &GuardContext<'_>) -> Result<bool, AppError> {
// //         let head = ctx.head();
// //         log::info!("head: {:?}", head);
// //         log::info!("query_params: {:?}", ctx);
// //         Ok(true)
// //     }
// // }

// use actix_web::guard::{Guard, GuardContext};
// use actix_web::web::Query;
// use actix_web::HttpResponse;
// use log::{error, info};
// use serde::{Deserialize, Serialize};
// use std::cell::RefCell;
// use std::rc::Rc;
// use validator::Validate;

// // 假设的 AppError 结构体
// #[derive(Debug, Serialize)]
// pub enum AppError {
//     BadRequest(String),
//     InternalServerError(String),
// }

// // 假设的 PaginationQuery 结构体
// #[derive(Debug, Serialize, Deserialize)]
// pub struct PaginationQuery {
//     pub page: Option<u64>,
//     pub limit: Option<u64>,
// }

// // 假设的 PaginatedResponse 结构体
// #[derive(Debug, Serialize)]
// pub struct PaginatedResponse {
//     pub data: Vec<UserEntity>,
//     pub pagination: PaginationInfo,
// }

// // 假设的 PaginationInfo 结构体
// #[derive(Debug, Serialize)]
// pub struct PaginationInfo {
//     pub total: u64,
//     pub total_pages: u64,
//     pub current_page: u64,
//     pub limit: u64,
//     pub has_next: bool,
//     pub has_previous: bool,
// }

// // 假设的 UserEntity 结构体
// #[derive(Debug, Serialize)]
// pub struct UserEntity {
//     pub id: u64,
//     pub name: String,
//     // 其他字段
// }

// // 假设的 ErrorResponse 结构体
// #[derive(Debug, Serialize)]
// pub struct ErrorResponse {
//     pub error: String,
// }

// // 假设的 DEFAULT_PAGE_SIZE 和 MAX_PAGE_SIZE 常量
// const DEFAULT_PAGE_SIZE: u64 = 10;
// const MAX_PAGE_SIZE: u64 = 100;

// // 泛型守卫结构体
// pub struct QueryGuard<Q>
// where
//     Q: serde::Deserialize<'static> + std::fmt::Debug + Validate,
// {
//     query: Query<Q>,
// }

// impl<Q> QueryGuard<Q>
// where
//     Q: serde::Deserialize<'static> + std::fmt::Debug + Validate,
// {
//     pub fn new(query: Query<Q>) -> Self {
//         QueryGuard { query }
//     }
// }

// impl<Q> Guard for QueryGuard<Q>
// where
//     Q: serde::Deserialize<'static> + std::fmt::Debug + Validate,
// {
//     fn check(&self, ctx: &GuardContext<'_>) -> bool {
//         log::error("QueryGuard check called with query: {:?}", self.query);
//         // match self.validate_query(&self.query.into_inner()) {
//         //     Ok(_) => true,
//         //     Err(err) => {
//         //         ctx.req_data_mut().insert(Rc::new(RefCell::new(Some(err))));
//         //         false
//         //     }
//         // }
//         true
//     }
// }

// // // 验证分页参数的逻辑
// // impl<Q, E> QueryGuard<Q, E>
// // where
// //     Q: serde::Deserialize<'static> + std::fmt::Debug,
// //     E: Serialize,
// // {
// //     fn validate_query(&self, query: &Q) -> Result<(), AppError> {
// //         // 这里可以根据具体的查询类型进行验证
// //         if let Some(pagination_query) = query.downcast_ref::<PaginationQuery>() {
// //             if pagination_query.page.is_none() || pagination_query.limit.is_none() {
// //                 return Err(AppError::BadRequest(
// //                     "分页参数 page 和 limit 必须提供".to_string(),
// //                 ));
// //             }

// //             let page = pagination_query.page.unwrap();
// //             let limit = pagination_query.limit.unwrap();

// //             if page < 1 {
// //                 return Err(AppError::BadRequest(
// //                     "分页参数 page 必须大于等于 1".to_string(),
// //                 ));
// //             }

// //             if limit < 1 {
// //                 return Err(AppError::BadRequest(
// //                     "分页参数 limit 必须大于等于 1".to_string(),
// //                 ));
// //             }

// //             info!("分页参数验证成功: page = {}, limit = {}", page, limit);
// //             Ok(())
// //         } else {
// //             Err(AppError::BadRequest("无效的查询参数类型".to_string()))
// //         }
// //     }
// // }

use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    guard::{Guard, GuardContext},
    http::header,
    Error, HttpMessage, HttpResponse,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::{ready, Ready};
use std::pin::Pin;
use std::rc::Rc;

use crate::AppError;

// 类型别名：校验函数 (参数名, 参数值) -> bool
type ValidatorFn = Box<dyn Fn(&str, &str) -> bool + Send + Sync>;

pub struct ParamGuard {
    validators: HashMap<String, ValidatorFn>,
    error_handler: Option<Box<dyn Fn() -> HttpResponse + Send + Sync>>,
}

impl ParamGuard {
    /// 创建守卫构造器
    pub fn builder() -> ParamGuardBuilder {
        ParamGuardBuilder::new()
    }
}

impl Guard for ParamGuard {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        log::error!("ParamGuard check called with query: {:?}", ctx.head());
        // 获取查询参数
        let query = ctx.head().uri.query().unwrap_or("");
        log::info!("query: {:?}", query);
        let params: HashMap<_, _> = url::form_urlencoded::parse(query.as_bytes())
            .into_owned()
            .collect();
        log::info!("Parsed parameters: {:?}", params);
        // 如果当前传入的参数在校验器中不存在，则返回错误响应
        // 检查是否存在未声明的参数
        for param_name in params.keys() {
            if !self.validators.contains_key(param_name) {
                log::warn!("意外参数: {}", param_name);
                ctx.req_data_mut()
                    .insert(Rc::new(RefCell::new(Some(AppError::Forbidden(
                        format!("未声明参数: {}", param_name).to_string(),
                    )))));
                return false; // 发现未声明参数立即返回失败
            }
        }
        // 遍历所有校验器
        for (param_name, validator) in &self.validators {
            log::info!("Checking parameter: {}", param_name);
            match params.get(param_name.as_str()) {
                Some(value) => {
                    log::info!("Found parameter: {}={}", param_name, value);
                    if !validator(param_name, value) {
                        log::warn!("Parameter validation failed: {}={}", param_name, value);
                        return false;
                    }
                }
                None => {
                    log::warn!("Required parameter missing: {}", param_name);
                    return false;
                }
            }
        }
        true
    }

    // fn on_reject(
    //     &self,
    //     ctx: &ServiceRequest,
    // ) -> Pin<Box<dyn futures::Future<Output = Result<ServiceResponse, Error>>>> {
    //     let response = match &self.error_handler {
    //         Some(handler) => handler(),
    //         None => HttpResponse::BadRequest().body("Invalid parameters"),
    //     };

    //     Box::pin(ready(Ok(req.into_response(response))))
    // }
}

/// 守卫构造器（支持链式调用）
pub struct ParamGuardBuilder {
    validators: HashMap<String, ValidatorFn>,
    error_handler: Option<Box<dyn Fn() -> HttpResponse + Send + Sync>>,
}

impl ParamGuardBuilder {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
            error_handler: None,
        }
    }

    /// 添加参数校验规则
    pub fn validate<P, V>(mut self, param: P, validator: V) -> Self
    where
        P: Into<String>,
        V: Fn(&str, &str) -> bool + 'static + Send + Sync,
    {
        self.validators.insert(param.into(), Box::new(validator));
        self
    }

    pub fn validate_or_default<F>(mut self, param: &str, validator: F, default: String) -> Self
    where
        F: Fn(&str, &str) -> bool + 'static + Send + Sync,
    {
        self.validators.insert(
            param.to_string(),
            Box::new(move |name, value| {
                log::info!("Validating parameter: {}", default);
                if value.is_empty() {
                    // 自动补充默认值到请求中
                    true
                } else {
                    validator(name, value)
                }
            }),
        );
        self
    }

    /// 设置自定义错误处理器
    pub fn error_handler<H>(mut self, handler: H) -> Self
    where
        H: Fn() -> HttpResponse + 'static + Send + Sync,
    {
        self.error_handler = Some(Box::new(handler));
        self
    }

    /// 构建最终守卫
    pub fn build(self) -> ParamGuard {
        ParamGuard {
            validators: self.validators,
            error_handler: self.error_handler,
        }
    }
}
