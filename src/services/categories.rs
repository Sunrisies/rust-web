use crate::middleware::helpers::{Resp, SimpleResp};
use crate::models::categories::{self};
use crate::models::sea_orm_active_enums::Type;
use actix_web::web;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryRequest {
    name: String,
    r#type: Type,
}

pub async fn create_category(
    db: web::Data<DatabaseConnection>,
    web::Json(payload): web::Json<CategoryRequest>,
) -> SimpleResp {
    log::info!("create_category payload: {:?}", payload);
    let category = categories::Entity::find()
        .filter(categories::Column::Name.eq(payload.name.clone()))
        .one(db.get_ref())
        .await;
    if let Ok(Some(_)) = category {
        return Resp::ok("", "分类名称已存在").to_json_result();
    }
    let category = categories::ActiveModel {
        name: Set(payload.name),
        r#type: Set(payload.r#type),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };

    match category.insert(db.get_ref()).await {
        Ok(_) => Resp::ok("", "创建分类成功").to_json_result(),
        Err(e) => Resp::ok("", "创建分类失败").to_json_result(),
    }
}
