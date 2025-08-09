use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterResponse {
    #[validate(length(min = 5, max = 100, message = "用户名长度必须在5到100之间"))]
    pub page: String,
    #[validate(length(min = 6, max = 100, message = "密码长度必须在6到100之间"))]
    pub limit: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct SluiceRequest {
    #[validate(custom(function = "validate_type"))]
    pub control_type: String,
    pub user_name: String,
    pub topic: String,
    pub device_id: String, // 手机设备id
}
// 验证函数，确保 type_ 字段的值是 "0"、"1" 或 "2"
pub fn validate_type(value: &str) -> Result<(), ValidationError> {
    match value {
        "0" | "1" | "2" => Ok(()),
        _ => Err(ValidationError::new("type must be 0, 1 or 2")),
    }
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct SluiceDevicesRequest {
    pub subscribe_topic: String,
    pub publish_topic: String,
    pub device_name: String,
    pub remark: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateSluiceDevicesRequest {
    pub subscribe_topic: Option<String>,
    pub publish_topic: Option<String>,
    pub device_name: Option<String>,
    pub remark: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}
