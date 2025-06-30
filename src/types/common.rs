use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Debug, Serialize, ToSchema, Deserialize, Clone)]
pub struct CommonResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize, ToSchema)]
pub struct PaginationInfo {
    pub total: u64,
    pub total_pages: u64,
    pub current_page: u64,
    pub limit: u64,
    pub has_next: bool,
    pub has_previous: bool,
}
