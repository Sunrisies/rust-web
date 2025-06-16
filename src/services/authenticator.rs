use crate::error::error::AppError;
use crate::models::user::{self, Entity as UserEntity};
use crate::utils::jsonwebtoken::{extract_token, has_permission};
use actix_web::HttpRequest;
use actix_web::{web, HttpResponse, Responder};
use base64::engine::general_purpose;
use base64::engine::Engine as _;
use google_authenticator::GoogleAuthenticator;
use image::*;
use log;
use qrcode::QrCode;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_json::json;
use std::io::Cursor;
// 添加用于处理2FA验证的端点
pub async fn verify_2fa(
    web::Json(data): web::Json<Verify2FARequest>,
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let auth = GoogleAuthenticator::new();
    if !auth.verify_code(&data.secret, &data.code, 1, 0) {
        log::debug!("2FA验证失败: code={}", data.code);
        return Err(AppError::BadRequest("无效的验证码".into()));
    }

    let token = extract_token(&req.headers())
        .await
        .ok_or_else(|| AppError::Unauthorized("请求未包含认证Token".into()))?;

    // 验证JWT是否具有权限
    let token_claims = has_permission(&token)
        .map_err(|e| AppError::Unauthorized(e.to_string()))?
        .claims;

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
    let code = QrCode::new(otp_uri.as_bytes())
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    let image = code.render::<Luma<u8>>().build();

    // 将二维码转换为PNG字节
    let png_bytes = write_image_to_png_bytes(image)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    // 转换为Base64
    let base64_image = general_purpose::STANDARD.encode(&png_bytes);
    let data_url = format!("data:image/png;base64,{}", base64_image);

    Ok(HttpResponse::Ok().json(json!( {
        "status": "success",
        "data": {
            "secret": secret,
            "qr_code": data_url
        }
    })))
}

fn write_image_to_png_bytes(
    image: ImageBuffer<Luma<u8>, Vec<u8>>,
) -> Result<Vec<u8>, image::ImageError> {
    let mut png_bytes = Vec::new();
    image.write_to(&mut Cursor::new(&mut png_bytes), ImageFormat::Png)?;
    Ok(png_bytes)
}

#[derive(Deserialize)]
pub struct Verify2FARequest {
    secret: String,
    code: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_generate_2fa_secret() {
        let mut app =
            test::init_service(App::new().route("/", web::get().to(generate_2fa_secret))).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

        let body: serde_json::Value = test::read_body_json(resp).await;

        // 测试密钥长度
        let secret = body["data"]["secret"].as_str().unwrap();
        assert_eq!(secret.len(), 32);

        // 测试OTP URI格式
        let otp_uri = body["data"]["qr_code"].as_str().unwrap();
        assert!(otp_uri.starts_with("data:image/png;base64,"));

        // 测试二维码生成和解析
        let base64_data = otp_uri.split(",").nth(1).unwrap();
        let png_bytes = general_purpose::STANDARD.decode(base64_data).unwrap();
        let img = image::load_from_memory(&png_bytes).unwrap();
        assert_eq!(img.width(), 360); // 假设二维码的默认宽度为360
        assert_eq!(img.height(), 360); // 假设二维码的默认高度为360
    }
}
