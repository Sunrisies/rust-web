use crate::error::error::AppError;
use crate::models::user::{self, Entity as UserEntity};
use crate::utils::jsonwebtoken::{extract_token, has_permission};
use actix_web::HttpRequest;
use actix_web::{web, HttpResponse, Responder};
use base64::engine::general_purpose;
use base64::engine::Engine as _;
use google_authenticator::GoogleAuthenticator;
use image::Luma;
use log;
use qrcode::QrCode;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_json::json;
// 添加用于处理2FA验证的端点
pub async fn verify_2fa(
    web::Json(data): web::Json<Verify2FARequest>,
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    // 1. 验证2FA代码
    let auth = GoogleAuthenticator::new();
    if !auth.verify_code(&data.secret, &data.code, 1, 0) {
        log::debug!("2FA验证失败: code={}", data.code);
        return Err(AppError::BadRequest("无效的验证码".into()));
    }

    // 2. 提取并验证JWT
    let token = extract_token(&req.headers())
        .await
        .ok_or_else(|| AppError::Unauthorized("请求未包含认证Token".into()))?;

    let token_claims = has_permission(&token)
        .map_err(|e| AppError::Unauthorized(e.to_string()))?
        .claims;

    // 3. 更新用户绑定信息
    let user = UserEntity::find_by_uuid(&token_claims.user_uuid)
        .one(db.as_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".into()))?;

    let mut user_active: user::ActiveModel = user.into();
    user_active.binding = Set(Some(data.secret));
    user_active.update(db.as_ref()).await?;

    log::info!("用户 {} 已激活2FA", token_claims.user_uuid);

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "双重验证已激活"
    })))
}

// 添加用于生成2FA密钥的端点
pub async fn generate_2fa_secret() -> Result<impl Responder, AppError> {
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
pub struct Verify2FARequest {
    secret: String,
    code: String,
}
