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
                        Ok(token_data) => token_data.claims.permissions,
                        Err(err) => {
                            eprintln!("Permission check error: {}", err);
                            return false;
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
                            // return stored_permissions.contains(*self.required_permission);
                            // 2. 宽松检查（包含任一要求的权限）
                            return stored_permissions.intersects(self.required_permission);
                        }else {
                            false
                        }
                    }else {
                        false
                    }
                    // 解包 permissions
                    //  if let Some(permissions_vec) = permissions {
                    //     // 检查权限是否包含 required_permission
                    //     if let Some(required_permission) = self.required_permission {
                    //         if permissions_vec.contains(&required_permission.to_string()) {
                    //             true
                    //         } else {
                    //             false
                    //         }
                    //     } else {
                    //         false
                    //     }
                    // } else {
                    //     false
                    // }
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
