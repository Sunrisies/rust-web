use crate::middleware::helpers::Resp;
use crate::models::sea_orm_active_enums::Type;
use crate::models::tags::{self, Entity as TagsEntity};
use crate::serde::deserialize_enum;
use crate::services::categories::SimpleRespData;
use actix_web::{web, Responder};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
