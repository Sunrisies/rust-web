use super::articles;
use super::auth;
use super::authenticator;
use super::categories;
use super::sse;
use super::user;
use crate::config::permission::Permission;
use crate::utils::permission_guard::PermissionGuard;
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/sse").route("/stream", web::get().to(sse::sse_stream)))
            .service(
                web::scope("/users")
                    .route("", web::get().to(user::get_all_users))
                    .route("/{uuid:.*}", web::put().to(user::update_user))
                    .route("/{uuid:.*}", web::get().to(user::get_user_by_uuid))
                    .route("/{uuid:.*}", web::delete().to(user::delete_user)),
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
                            .to(articles::get_article),
                    )
                    .route(
                        web::post()
                            .guard(PermissionGuard::new(Permission::WRITE_ARTICLE))
                            .to(articles::create_article),
                    ),
            )
            .service(
                web::scope("/2fa")
                    .route("/verify", web::post().to(authenticator::verify_2fa))
                    .route(
                        "/generate",
                        web::get().to(authenticator::generate_2fa_secret),
                    ),
            )
            .service(
                web::scope("/categories").route("", web::post().to(categories::create_category)),
            ),
    );
}
