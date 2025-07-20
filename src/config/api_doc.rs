use crate::services::auth;
// use crate::services::user;
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
        // 权限模块的
        auth::register, // 注册
        // auth::login, // 登录


        // 用户模块
        // user::get_all_users, // 获取全部用户
        // user::get_user_by_uuid, // 根据UUID获取用户信息
        // user::delete_user,  // 删除用户
        // user::update_user, // 更新用户信息
    )
)]
pub struct ApiDoc;

// #[cfg(debug_assertions)]
pub fn write_to_file() {
    let openapi_json = ApiDoc::openapi().to_pretty_json().unwrap();
    let mut file = File::create("openapi.json").unwrap();
    writeln!(file, "{}", openapi_json).unwrap();
    log::info!("OpenAPI JSON written to openapi.json");
    // log::info!("{}112112312", openapi_json);
}
