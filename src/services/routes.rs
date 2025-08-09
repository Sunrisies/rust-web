use super::auth;
use super::sluice;
use super::sluice_devices;
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
            )
            .service(
                web::scope("/sluice")
                    .route("/listSluiceData", web::get().to(sluice::get_all_sluice))
                    .route("/getSluiceLastData", web::get().to(sluice::get_sluice))
                    .route(
                        "/listSluiceControlLog",
                        web::get().to(sluice::get_all_sluice_control_log),
                    )
                    .route("/control", web::post().to(sluice::control_sluice))
                    .route(
                        "/sluiceDevices",
                        web::post().to(sluice_devices::create_sluice_devices),
                    )
                    .route(
                        "/sluiceDevices",
                        web::get().to(sluice_devices::get_sluice_devices),
                    )
                    .route(
                        "/sluiceDevices/{id}",
                        web::delete().to(sluice_devices::delete_sluice_devices),
                    )
                    .route(
                        "/sluiceDevices/{id}",
                        web::put().to(sluice_devices::update_sluice_devices),
                    ),
            ),
    );
}
