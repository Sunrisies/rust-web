use crate::{
    config::permission::Permission,
    jsonwebtoken::{extract_token, has_permission},
    AppError,
};
use actix_web::guard::{Guard, GuardContext};
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
        let token = extract_token(&head.headers);
        let token_data = match token {
            Some(token_str) => has_permission(&token_str).map_err(|err| {
                eprintln!("权限检查错误: {}", err);
                AppError::Forbidden(err.to_string())
            })?,
            None => {
                error!("令牌未找到");
                return Err(AppError::Forbidden("令牌未找到".to_string()));
            }
        };

        let permissions = token_data.claims.permissions.ok_or_else(|| {
            error!("权限字符串为空");
            AppError::Forbidden("权限字符串为空".to_string())
        })?;

        let permissions_bits = permissions.parse::<u64>().map_err(|_| {
            error!("权限字符串格式错误");
            AppError::Forbidden("权限字符串格式错误".to_string())
        })?;

        let stored_permissions =
            Permission::from_bits(permissions_bits).unwrap_or(Permission::NONE);
        info!(
            "存储的权限: {:?}, 必需的权限: {:?}",
            stored_permissions, self.required_permission
        );

        if stored_permissions.intersects(self.required_permission) {
            Ok(true)
        } else {
            Err(AppError::Forbidden("权限不足".to_string()))
        }
    }
}
