use crate::common::{
    CategoryQuery, CommonResponse, PaginatedResponse, PaginationInfo, PaginationQuery,
    DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE,
};
use crate::data_processing::deep_filter_data;
use crate::middleware::helpers::{Resp, SimpleResp};
use crate::models::categories::{self, Entity as CategoriesEntity};
use crate::models::sea_orm_active_enums::Type;
use crate::serde::deserialize_enum;
use crate::serde::EnumDeserialize;
use crate::services::user::UserInfo;
use crate::utils::query_parameter::Query;
use crate::AppError;
use actix_web::web;
use chrono::Utc;
use log::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

impl EnumDeserialize for Type {
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "Article" => Ok(Type::Article),
            "Library" => Ok(Type::Library),
            _ => Err(()),
        }
    }

    fn valid_values() -> Vec<&'static str> {
        vec!["Article", "Library"]
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryRequest {
    name: String,
    #[serde(deserialize_with = "deserialize_enum")]
    r#type: Type,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SimpleRespData {
    data: String,
    message: String,
}
#[utoipa::path(
    post,
    path = "/api/categories",
    request_body = CategoryRequest,
    tag = "分类",
    operation_id = "创建分类",
    responses(
        (status = 200, description = "创建分类成功", body = SimpleRespData),
        (status = 400, description = "分类名称已存在", body = SimpleRespData),
        (status = 500, description = "创建分类失败", body = SimpleRespData),
    ),
)]
pub async fn create_category(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<CategoryRequest>,
) -> SimpleResp {
    log::info!("create_category payload: {:?}", payload);
    let category = CategoriesEntity::find()
        .filter(categories::Column::Name.eq(payload.name.clone()))
        .one(db.get_ref())
        .await;
    if let Ok(Some(_)) = category {
        return Resp::ok("", "分类名称已存在").to_json_result();
    }
    let category = categories::ActiveModel {
        name: Set(payload.name.clone()),
        r#type: Set(payload.r#type.clone()),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };

    match category.insert(db.get_ref()).await {
        Ok(data) => Resp::ok(data, "创建分类成功").to_json_result(),
        Err(e) => {
            log::error!("create_category error: {}", e);
            Resp::ok("", "创建分类失败").to_json_result()
        }
    }
}

// 获取分类列表,带有分页
#[utoipa::path(
    get,
    path = "/api/categories",
    tag = "分类",
    operation_id = "获取分类列表",
    request_body = PaginationQuery,
    responses(
        (status = 200, description = "获取分类列表成功", body = CommonResponse<UserInfo>),
        (status = 500, description = "获取分类列表失败", body = SimpleRespData),
    ),
)]
pub async fn get_all_categories(
    db: web::Data<DatabaseConnection>,
    query: Query<CategoryQuery>,
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
    let (total, categories) = match tokio::try_join!(
        CategoriesEntity::find().count(db.as_ref()),
        CategoriesEntity::find()
            .order_by_desc(categories::Column::Id)
            .offset(Some(offset))
            .limit(Some(limit))
            .all(db.as_ref())
    ) {
        Ok((total, categories)) => (total, categories),
        Err(e) => {
            error!("数据库操作失败: {}", e);
            return Resp::err(AppError::InternalServerError("数据库操作失败".to_string()))
                .to_json_result();
        }
    };
    let total_pages = (total + limit - 1) / limit; // 整数除法避免浮点误差
    let data = deep_filter_data(categories, vec!["id"]);
    // // 获取分页用户数据
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

// 删除分类
#[utoipa::path(
    delete,
    path = "/api/categories/{id}",
    tag = "分类",
    operation_id = "删除分类",
    responses(
        (status = 200, description = "删除分类成功", body = SimpleRespData),
        (status = 404, description = "分类不存在", body = SimpleRespData),
        (status = 500, description = "删除分类失败", body = SimpleRespData),
    ),
)]
pub async fn delete_category(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> SimpleResp {
    let category = match CategoriesEntity::find_by_id(id.into_inner())
        .one(db.get_ref())
        .await
    {
        Ok(data) => data,
        Err(e) => {
            log::error!("delete_category error: {}", e);
            return Resp::err(AppError::InternalServerError("查询分类失败".to_string()))
                .to_json_result();
        }
    };
    if category.is_none() {
        return Resp::err(AppError::NotFound("分类不存在".to_string())).to_json_result();
    }
    let category = category.unwrap();
    match category.delete(db.get_ref()).await {
        Ok(_) => Resp::ok("", "删除分类成功").to_json_result(),
        Err(e) => {
            log::error!("delete_category error: {}", e);
            Resp::ok("", "删除分类失败").to_json_result()
        }
    }
}
