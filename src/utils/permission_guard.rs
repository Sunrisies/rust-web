use crate::{config::permission::Permission, jsonwebtoken::has_permission, AppError};
use actix_web::{
    guard::{Guard, GuardContext},
    http::header,
};
use log::{error, info};
use std::cell::RefCell;
use std::rc::Rc;

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
        match self.check_permission(ctx) {
            Ok(result) => result,
            Err(err) => {
                ctx.req_data_mut().insert(Rc::new(RefCell::new(Some(err))));
                false
            }
        }
    }
}

impl PermissionGuard {
    fn check_permission(&self, ctx: &GuardContext<'_>) -> Result<bool, AppError> {
        let head = ctx.head();
        if let Some(auth_header) = head.headers.get(header::AUTHORIZATION) {
            if let Ok(token_str) = auth_header.to_str() {
                if let Some(token) = token_str.strip_prefix("Bearer ") {
                    // 解码令牌并获取权限
                    let permissions = match has_permission(token) {
                        Ok(token_data) => token_data.claims.permissions,
                        Err(err) => {
                            eprintln!("Permission check error: {}", err);
                            return Err(AppError::Forbidden(err.to_string()));
                        }
                    };

                    // 对比权限
                    if let Some(permissions_str) = permissions {
                        if let Ok(permissions_bits) = permissions_str.parse::<u64>() {
                            let stored_permissions =
                                Permission::from_bits(permissions_bits).unwrap_or(Permission::NONE);
                            info!(
                                "Stored permissions: {:?}, Required permissions: {:?}",
                                stored_permissions, self.required_permission
                            );

                            // 返回检查结果
                            if stored_permissions.intersects(self.required_permission) {
                                Ok(true)
                            } else {
                                Err(AppError::Forbidden("权限不足11".to_string()))
                            }
                        } else {
                            Err(AppError::Forbidden("权限字符串格式错误".to_string()))
                        }
                    } else {
                        Err(AppError::Forbidden("权限字符串为空".to_string()))
                    }
                } else {
                    error!("令牌格式不正确");
                    Err(AppError::Forbidden("令牌格式不正确".to_string()))
                }
            } else {
                error!("token not found");
                Err(AppError::Forbidden("token not found".to_string()))
            }
        } else {
            error!("Authorization header not found");
            Err(AppError::Forbidden(
                "Authorization header not found".to_string(),
            ))
        }
    }
}
