use crate::AppError;
use actix_web::guard::{Guard, GuardContext};
use log::{error, info};
use std::cell::RefCell;
use std::rc::Rc;

pub struct PaginationGuard;

impl Guard for PaginationGuard {
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

impl PaginationGuard {
    fn check_permission(&self, ctx: &GuardContext<'_>) -> Result<bool, AppError> {
        let head = ctx.head();
        log::info!("head: {:?}", head);
        log::info!("query_params: {:?}", ctx);
        Ok(true)
    }
}
