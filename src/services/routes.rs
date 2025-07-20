use super::auth;
use super::sse;
use super::user;
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/sse").route("/stream", web::get().to(sse::sse_stream)))
            .service(
                web::scope("/users")
                    .route("", web::get().to(user::get_all_users))
                    .route("/{uuid:.*}", web::put().to(user::update_user))
                    .route("/{uuid:.*}", web::delete().to(user::delete_user)),
            )
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(auth::login))
                    .route("/register", web::post().to(auth::register)),
            ),
    );
}
