use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    #[sea_orm(unique)]
    pub user_name: String,
    pub pass_word: String,
    pub email: Option<String>,
    pub age: Option<i32>,
    pub image: Option<String>,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub permissions: Option<String>,
    pub binding: Option<String>,
    #[sea_orm(default_value_t = DateTimeUtc::default())]
    pub created_at: DateTimeUtc,
    #[sea_orm(default_value_t = DateTimeUtc::default())]
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
