use super::auth;
use super::user;
use actix_web::web;
pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/users")
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
