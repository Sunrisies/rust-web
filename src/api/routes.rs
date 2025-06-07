use super::auth;
use super::user;
use actix_web::dev::ServiceRequest;
use actix_web::guard::Guard;
use actix_web::guard::GuardContext;
use actix_web::web;
use log::info;
pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/users")
                    .guard(SimpleGuard)
                    .route("", web::get().to(user::get_all_users))
                    // .route("", web::post().to(user::create_user))
                    .route("/{uuid}", web::get().to(user::get_user_by_uuid))
                    .route("/{id}", web::put().to(user::update_user))
                    .route("/{id}", web::delete().to(user::delete_user)),
            )
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(auth::login))
                    .route("/register", web::post().to(auth::register)),
            ),
    );
}
// 定义一个简单的守卫
struct SimpleGuard;

impl Guard for SimpleGuard {
    fn check(&self, ctx: &GuardContext) -> bool {
        // 在这里添加你的守卫逻辑
        // 例如检查请求头中是否包含特定的令牌
        // 现在假设用户是超级管理员，权限都有
        let user_id = 1;
        info!("SimpleGuard check{:?}", ctx);
        // ctx.header();

        true
        // 检查请求头中的 `X-API-Key` 是否等于 `"secret"`
        // ctx.headers()
        //     .get("X-API-Key")
        //     .map_or(false, |v| v == "secret")
    }
}
