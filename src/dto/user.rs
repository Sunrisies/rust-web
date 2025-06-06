use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    #[validate(length(min = 10, max = 100, message = "用户名长度必须在10到100之间"))]
    #[serde(rename = "user_name")]
    pub user_name: String,
    #[validate(
        length(min = 1, message = "电子邮件是必需的"),
        email(message = "电子邮件无效")
    )]
    #[serde(rename = "email")]
    pub email: String,
    pub age: Option<i32>,
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

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    #[validate(length(min = 10, max = 100, message = "用户名长度必须在10到100之间"))]
    pub user_name: String,
    #[validate(length(min = 6, max = 100, message = "密码长度必须在6到100之间"))]
    pub pass_word: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    #[validate(length(min = 10, max = 100, message = "用户名长度必须在10到100之间"))]
    pub user_name: String,
    #[validate(length(min = 6, max = 100, message = "密码长度必须在6到100之间"))]
    pub pass_word: String,
}
