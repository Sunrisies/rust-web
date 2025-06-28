use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    #[validate(length(min = 10, max = 100, message = "用户名长度必须在5到100之间"))]
    #[serde(rename = "user_name")]
    pub user_name: String,
    #[validate(
        length(min = 1, message = "电子邮件是必需的"),
        email(message = "电子邮件无效")
    )]
    #[serde(rename = "email")]
    pub email: String,
    #[validate(length(min = 6, max = 100, message = "密码长度必须在6到100之间"))]
    #[serde(rename = "pass_word")]
    pub pass_word: String,
    #[serde(rename = "image")]
    pub image: Option<String>,
    #[serde(rename = "phone")]
    pub phone: Option<String>,
    #[serde(rename = "role")]
    pub role: Option<String>,
    #[serde(rename = "permissions")]
    pub permissions: Option<String>,
    #[serde(rename = "binding")]
    pub binding: Option<String>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterResponse {
    #[validate(length(min = 5, max = 100, message = "用户名长度必须在5到100之间"))]
    pub user_name: String,
    #[validate(length(min = 6, max = 100, message = "密码长度必须在6到100之间"))]
    pub pass_word: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[validate(length(min = 5, max = 100, message = "用户名长度必须在5到100之间"))]
    pub user_name: String,
    #[validate(length(min = 6, max = 100, message = "密码长度必须在6到100之间"))]
    pub pass_word: String,
}

#[derive(Deserialize, Debug, Default, Clone, Serialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 5, max = 100, message = "用户名长度必须在5到100之间"))]
    pub user_name: String,
    #[serde(rename = "image")]
    pub image: Option<String>,
    pub permissions: Option<Vec<String>>,
}
