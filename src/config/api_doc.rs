use crate::services::auth;
use crate::services::categories;
use crate::services::user;
use std::fs::File;
use std::io::Write;
use utoipa::OpenApi;

#[derive(OpenApi)]
// #[openapi(paths(categories::create_category, auth::register,))]
#[openapi(
    info(
        title = "Rust Web API",
        version = "1.0",
        description = "一个简单的Rust web API",
        terms_of_service = "https://www.rust-web-api.com/terms",
        contact(
            name = "Sunrisies",
            email = "3266420686@qq.com",
            url = "https://github.com/Sunrisies/rust-web"
        ),
    ),
    paths(
        // 分类模块
        categories::create_category,

        // 权限模块的
        auth::register,
        auth::get_permissions,
        auth::login,
        auth::get_permissions_by_id,


        // 用户模块
        user::get_all_users
    )
)]
pub struct ApiDoc;

#[cfg(debug_assertions)]
pub fn write_to_file() {
    let openapi_json = ApiDoc::openapi().to_pretty_json().unwrap();
    let mut file = File::create("openapi.json").unwrap();
    writeln!(file, "{}", openapi_json).unwrap();
    log::info!("{}112112312", openapi_json);
}
