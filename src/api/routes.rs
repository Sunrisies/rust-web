use super::auth;
use super::user;
use crate::config::permission::Permission;
use crate::error::error::AppError;
use crate::utils::permission_guard::PermissionGuard;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use base64::engine::general_purpose;
use base64::engine::Engine as _;
use google_authenticator::GoogleAuthenticator;
use image::Luma;
use qrcode::QrCode;
use serde::Deserialize;
use serde_json::json;
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
            .service(
                web::scope("/users")
                    // .guard(SimpleGuard)
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
            )
            .service(
                web::resource("/articles")
                    .route(web::get().to(get_article))
                    .route(
                        web::post()
                            .guard(PermissionGuard::new(Permission::WRITE_ARTICLE))
                            .to(create_article),
                    ),
            )
            .service(
                web::scope("/2fa")
                    .route("/verify", web::post().to(verify_2fa))
                    .route("/generate", web::get().to(generate_2fa_secret)),
            ),
    );
}
// 定义一个简单的守卫
// struct PermissionGuard;

// impl Guard for PermissionGuard {
//     fn new(permission: Permission) -> Self {
//         Self
//     }
//     fn check(&self, ctx: &GuardContext) -> bool {
//         // 在这里添加你的守卫逻辑
//         // 例如检查请求头中是否包含特定的令牌
//         // 现在假设用户是超级管理员，权限都有
//         let user_id = 1;
//         info!("SimpleGuard check{:?}", ctx);
//         // ctx.header();

//         true
//         // 检查请求头中的 `X-API-Key` 是否等于 `"secret"`
//         // ctx.headers()
//         //     .get("X-API-Key")
//         //     .map_or(false, |v| v == "secret")
//     }
// }

// 添加用于处理2FA验证的端点
async fn verify_2fa(
    web::Json(data): web::Json<Verify2FARequest>,
) -> Result<impl Responder, AppError> {
    let auth = GoogleAuthenticator::new();
    let is_valid = auth.verify_code(&data.secret, &data.code, 1, 0);

    if is_valid {
        Ok(HttpResponse::Ok().json(json!({"status": "success"})))
    } else {
        Ok(HttpResponse::BadRequest().json(json!({"error": "Invalid code"})))
    }
}

// 添加用于生成2FA密钥的端点
async fn generate_2fa_secret() -> Result<impl Responder, AppError> {
    let auth = GoogleAuthenticator::new();
    let secret = auth.create_secret(32); // 生成32字节的密钥

    // 生成OTP URI
    let otp_uri = format!(
        "otpauth://totp/MyApp:UserAccount?secret={}&issuer=MyApp",
        secret
    );

    // 生成二维码
    let code = QrCode::new(otp_uri.as_bytes()).unwrap();
    let image = code.render::<Luma<u8>>().build();

    // 将二维码转换为PNG字节
    let mut png_bytes = Vec::new();
    image::DynamicImage::ImageLuma8(image)
        .write_to(
            &mut std::io::Cursor::new(&mut png_bytes),
            image::ImageFormat::Png,
        )
        .unwrap();

    // 转换为Base64
    let base64_image = general_purpose::STANDARD.encode(&png_bytes);
    let data_url = format!("data:image/png;base64,{}", base64_image);

    Ok(HttpResponse::Ok().json(json!({
        "secret": secret,
        "qr_code": data_url
    })))
}
#[derive(Deserialize)]
struct Verify2FARequest {
    secret: String,
    code: String,
}
