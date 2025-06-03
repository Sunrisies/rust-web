use actix_web::web;
use super::handlers;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/users")
                    .route("", web::get().to(handlers::get_all_users))
                    .route("", web::post().to(handlers::create_user))
                    .route("/{id}", web::get().to(handlers::get_user_by_id))
                    .route("/{id}", web::put().to(handlers::update_user))
                    .route("/{id}", web::delete().to(handlers::delete_user))
            )
    );
}