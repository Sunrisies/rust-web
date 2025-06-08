use crate::config::permission::Permission;
use actix_web::{guard::Guard, guard::GuardContext};
use log::{error, info};
pub struct PermissionGuard {
    required_permission: Permission,
}

impl PermissionGuard {
    pub fn new(required_permission: Permission) -> Self {
        PermissionGuard {
            required_permission,
        }
    }
}
impl Guard for PermissionGuard {
    fn check(&self, ctx: &GuardContext) -> bool {
        // 从请求中获取用户权限（实际项目中可能从数据库或上下文中获取）
        // let user_permission = get_user_permission(&req);
        info!("PermissionGuard check{:?}", ctx);
        error!("PermissionGuard check{:?}", &self.required_permission);
        true
    }
}

// impl actix_web::Handler<actix_web::dev::ServiceRequest, actix_web::dev::ServiceResponse>
//     for PermissionGuard
// {
//     type Error = actix_web::Error;
//     type Future = actix_web::dev::ServiceResponse;

//     fn handle(
//         &mut self,
//         req: actix_web::dev::ServiceRequest,
//         srv: &mut actix_web::dev::Service,
//     ) -> Result<Self::Future, Self::Error> {
//         // 从请求中获取用户权限（实际项目中可能从数据库或上下文中获取）
//         let user_permission = get_user_permission(&req);

//         // 检查权限
//         if user_permission.contains(self.required_permission) {
//             // 如果有权限，继续处理请求
//             Ok(srv.call(req).wait()?)
//         } else {
//             // 如果没有权限，返回 403 禁止访问
//             Err(actix_web::error::ErrorForbidden("无权限访问"))
//         }
//     }
// }

// 模拟从请求中获取用户权限
// fn get_user_permission(req: &actix_web::dev::ServiceRequest) -> Permission {
//     // 实际项目中从数据库或上下文中获取用户权限
//     // 这里只是一个示例
//     Permission::READ_WRITE_ARTICLE
// }
