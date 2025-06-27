use crate::middleware::helpers::{Resp, SimpleResp};
use crate::models::categories::{self};
use crate::models::sea_orm_active_enums::Type;
use crate::serde::deserialize_enum;
use crate::serde::EnumDeserialize;
use actix_web::web;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryRequest {
    name: String,
    #[serde(deserialize_with = "deserialize_enum")]
    r#type: Type,
}

pub async fn create_category(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<CategoryRequest>,
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
