use crate::common::{
    CommonResponse, PaginatedResponse, PaginationInfo, PaginationQuery, DEFAULT_PAGE_SIZE,
    MAX_PAGE_SIZE,
};
use crate::config::permission::{Permission, PERMISSION_MAP};
use crate::data_processing::{deep_filter_data, filter_value};
use crate::dto::user::{UpdateUserRequest, UserDto};
use crate::error::error::AppError;
use crate::middleware::helpers::{Resp, SimpleResp};
use crate::models::article::{self, Entity as ArticleEntity};
use crate::models::user::{self, Entity as UserEntity};
use crate::utils::query_parameter::Query;
use crate::utils::sse::SseNotifier;
use actix_web::web;
use actix_web::HttpResponse;
use chrono::Utc;
use log::{error, info};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid; // 添加uuid crate依赖
use validator::Validate;
// 示例接口
pub async fn get_article(
    db: web::Data<DatabaseConnection>,
    query: Query<PaginationQuery>,
) -> SimpleResp {
    // 验证分页参数
    let validated_query = match query.validate() {
        Ok(_) => query.into_inner(),
        Err(e) => {
            log::error!("分页参数验证失败: {:?}", e);

            return Resp::err(AppError::DatabaseError("分页参数验证失败".to_string()))
                .to_json_result();
        }
    };
    let page = validated_query.page.unwrap_or(1);
    let limit = validated_query.limit.unwrap_or(DEFAULT_PAGE_SIZE);

    // 限制每页数量的范围
    let limit = if limit == 0 {
        DEFAULT_PAGE_SIZE
    } else if limit > MAX_PAGE_SIZE {
        MAX_PAGE_SIZE
    } else {
        limit
    };
    let offset = (page - 1) * limit;

    // 获取总数和分页数据
    let (total, articles) = tokio::try_join!(
        ArticleEntity::find().count(db.as_ref()),
        ArticleEntity::find()
            .order_by_desc(article::Column::Id)
            .offset(Some(offset))
            .limit(Some(limit))
            .all(db.as_ref())
    )
    .map_err(|e| {
        error!("数据库操作获取文章列表失败: {}", e);
        AppError::DatabaseError("服务器异常，请联系管理员".to_string())
    })?;
    let total_pages = (total + limit - 1) / limit; // 整数除法避免浮点误差

    info!("total1: {}, articles: {:?}, ", total, articles);
    let data = deep_filter_data(articles, vec!["size"]);
    // 获取分页用户数据
    let response = PaginatedResponse {
        data,
        pagination: PaginationInfo {
            total,
            total_pages,
            current_page: page,
            limit,
            has_next: page < total_pages,
            has_previous: page > 1,
        },
    };

    Resp::ok(response, "获取用户列表成功").to_json_result()
}

pub async fn create_article() -> HttpResponse {
    //
    HttpResponse::Ok().body("创建文章")
}
