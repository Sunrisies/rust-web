use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    #[validate(length(min = 10, max = 100, message = "用户名长度必须在10到100之间"))]
    pub username: String,
    #[validate(
        length(min = 1, message = "电子邮件是必需的"),
        email(message = "电子邮件无效")
    )]
    pub email: String,
    pub age: Option<i32>,
    #[validate(length(min = 6, max = 100, message = "密码长度必须在6到100之间"))]
    pub password: String,
}
