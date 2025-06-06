use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct CommonResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: T,
}
