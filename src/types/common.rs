use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

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

pub const DEFAULT_PAGE_SIZE: u64 = 10;
pub const MAX_PAGE_SIZE: u64 = 100;
#[derive(Validate, Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    #[validate(range(min = 1, message = "页码必须大于1"))]
    pub page: Option<u64>,
    // 每页数量不能超过100
    #[serde(default = "default_size")]
    #[validate(range(max = MAX_PAGE_SIZE, message = "每页数量不能超过100"))]
    pub limit: Option<u64>,
}

// 添加默认函数实现
fn default_page() -> Option<u64> {
    Some(1)
}

fn default_size() -> Option<u64> {
    Some(DEFAULT_PAGE_SIZE)
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
}

// #[macro_export]
// 定义宏来生成包含分页参数的结构体
macro_rules! paginated_query {
    ($struct_name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        #[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
        pub struct $struct_name {
            // 分页参数（直接包含，非嵌套）
            #[serde(default = "default_page")]
            #[validate(range(min = 1, message = "页码必须大于1"))]
            pub page: Option<u64>,

            #[serde(default = "default_size")]
            #[validate(range(max = MAX_PAGE_SIZE, message = "每页数量不能超过100"))]
            pub limit: Option<u64>,

            // 自定义字段
            $(
                pub $field: $type,
            )*
        }
    };
}

paginated_query!(CategoryQuery {
    select: Option<String>
});
