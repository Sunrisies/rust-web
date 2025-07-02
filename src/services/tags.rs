use crate::common::{
    CommonResponse, PaginatedResponse, PaginationInfo, PaginationQuery, TagsQuery,
    DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE,
};
use crate::data_processing::deep_filter_data;
use crate::middleware::helpers::{Resp, SimpleResp};
use crate::models::sea_orm_active_enums::Type;
use crate::models::tags::{self, Entity as TagsEntity};
use crate::serde::deserialize_enum;
use crate::services::categories::SimpleRespData;
use crate::services::user::UserInfo;
use crate::utils::query_parameter::Query;
use crate::AppError;
use actix_web::{web, Responder};
use chrono::Utc;
use log::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct CreateTagRequest {
    pub name: String,
    #[serde(deserialize_with = "deserialize_enum")]
    r#type: Type,
}

#[utoipa::path(
    post,
    path = "/tags",
    request_body = CreateTagRequest,
    responses(
        (status = 201, description = "Tag created successfully", body = SimpleRespData),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_tag(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<CreateTagRequest>,
) -> impl Responder {
    log::info!("create_category payload: {:?}", payload);
    let category = TagsEntity::find()
        .filter(tags::Column::Name.eq(payload.name.clone()))
        .one(db.get_ref())
        .await;
    if let Ok(Some(_)) = category {
        return Resp::ok("", "标签名称已存在").to_json_result();
    }
    let tags = tags::ActiveModel {
        name: Set(payload.name.clone()),
        r#type: Set(payload.r#type.clone()),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };

    match tags.insert(db.get_ref()).await {
        Ok(data) => Resp::ok(data, "创建标签成功").to_json_result(),
        Err(e) => {
            log::error!("create_category error: {}", e);
            Resp::ok("", "创建标签失败").to_json_result()
        }
    }
}
// 获取标签列表,带有分页
#[utoipa::path(
    get,
    path = "/api/tags",
    tag = "标签",
    operation_id = "获取标签列表",
    request_body = PaginationQuery,
    responses(
        (status = 200, description = "获取标签列表成功", body = CommonResponse<UserInfo>),
        (status = 500, description = "获取标签列表失败", body = SimpleRespData),
    ),
)]
pub async fn get_all_tags(
    db: web::Data<DatabaseConnection>,
    query: Query<TagsQuery>,
) -> SimpleResp {
    // 验证分页参数
    let validated_query = match query.validate() {
        Ok(_) => query.into_inner(),
        Err(e) => {
            error!("分页参数验证失败: {:?}", e);

            return Resp::err(AppError::DeserializeError("分页参数验证失败".to_string()))
                .to_json_result();
        }
    };
    info!("validated_query: {:?}", validated_query);
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
    let (total, tags) = match tokio::try_join!(
        TagsEntity::find().count(db.as_ref()),
        TagsEntity::find()
            .order_by_desc(tags::Column::Id)
            .offset(Some(offset))
            .limit(Some(limit))
            .all(db.as_ref())
    ) {
        Ok((total, tags)) => (total, tags),
        Err(e) => {
            error!("数据库操作失败: {}", e);
            return Resp::err(AppError::InternalServerError("数据库操作失败".to_string()))
                .to_json_result();
        }
    };
    let total_pages = (total + limit - 1) / limit; // 整数除法避免浮点误差
    let data = deep_filter_data(tags, vec!["id"]);

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

    Resp::ok(response, "获取标签列表成功").to_json_result()
}
