use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Debug, Serialize, ToSchema, Deserialize, Clone)]
pub struct CommonResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: T,
}
