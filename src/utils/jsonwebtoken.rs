use actix_web::http::header::HeaderMap;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use log::error;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_uuid: String,
    pub user_name: String,
    pub exp: usize, // 令牌过期时间
}

pub fn has_permission(token: &str) -> Result<TokenData<TokenClaims>, Box<dyn std::error::Error>> {
    let token_message = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret("secret_key".as_bytes()),
        &Validation::new(Algorithm::HS256),
    );

    match token_message {
        Ok(token_data) => Ok(token_data),
        Err(err) => {
            // 处理解码错误
            error!("解码令牌时发生错误: {:?}", err);
            Err(err.into())
        }
    }
}

pub fn extract_token(headers: &HeaderMap) -> Option<String> {
    if let Some(authorization_header) = headers.get("Authorization") {
        if let Ok(authorization_str) = authorization_header.to_str() {
            // 假设令牌格式为 "Bearer <token>"
            if let Some(token) = authorization_str.strip_prefix("Bearer ") {
                return Some(token.to_string());
            }
        }
    }
    None
}
