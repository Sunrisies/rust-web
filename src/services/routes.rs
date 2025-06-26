use super::auth;
use super::authenticator;
use super::sse;
use super::user;
// use crate::common_guard::ParamGuard;
// use crate::common_guard::QueryGuard;
use crate::config::permission::Permission;
use crate::utils::permission_guard::PermissionGuard;
use actix_web::web;
use actix_web::HttpResponse;
// 示例接口
async fn get_article() -> HttpResponse {
    HttpResponse::Ok().body("文章列表")
}

async fn create_article() -> HttpResponse {
    HttpResponse::Ok().body("创建文章")
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/sse").route("/stream", web::get().to(sse::sse_stream)))
            .service(
                web::scope("/users")
                    .route("", web::get().to(user::get_all_users))
                    .route("/{uuid}", web::put().to(user::update_user))
                    .route("/{uuid}", web::get().to(user::get_user_by_uuid))
                    .route("/{uuid}", web::delete().to(user::delete_user)),
            )
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(auth::login))
                    .route("/register", web::post().to(auth::register))
                    .route("/permissions", web::get().to(auth::get_permissions))
                    .route("/permission", web::get().to(auth::get_permissions_by_id)),
            )
            .service(
                web::resource("/articles")
                    .route(
                        web::get()
                            .guard(PermissionGuard::new(Permission::READ_ARTICLE))
                            .to(get_article),
                    )
                    .route(
                        web::post()
                            .guard(PermissionGuard::new(Permission::WRITE_ARTICLE))
                            .to(create_article),
                    ),
            )
            .service(
                web::scope("/2fa")
                    .route("/verify", web::post().to(authenticator::verify_2fa))
                    .route(
                        "/generate",
                        web::get().to(authenticator::generate_2fa_secret),
                    ),
            ),
    );
}
