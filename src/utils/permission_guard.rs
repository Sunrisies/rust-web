use crate::{config::permission::Permission, jsonwebtoken::has_permission, AppError};
use actix_web::{guard::Guard, guard::GuardContext, http::header};
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
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        let head = ctx.head();
        if let Some(auth_header) = head.headers.get(header::AUTHORIZATION) {
            if let Ok(token_str) = auth_header.to_str() {
                if let Some(token) = token_str.strip_prefix("Bearer ") {
                    // 解码令牌并获取权限
                    let permissions = match has_permission(token) {
                        Ok(token_data) => Ok(token_data.claims.permissions),
                        Err(err) => Err(err),
                    };
                    permissions.contains(&self.required_permission)
                } else {
                    error!("令牌格式不正确");
                    false
                }
            } else {
                error!("token not found");
                false
            }
        } else {
            false
        }
    }
}
