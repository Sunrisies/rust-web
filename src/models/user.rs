use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "t_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub user_name: String,
    pub pass_word: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    #[sea_orm(default_value_t = DateTimeUtc::default())]
    pub create_time: DateTimeUtc,
    pub status: i8,
    #[sea_orm(default_value_t = DateTimeUtc::default())]
    pub update_time: DateTimeUtc,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Entity {}

impl From<Model> for JsonValue {
    fn from(model: Model) -> JsonValue {
        serde_json::to_value(model).unwrap()
    }
}

impl TryFrom<JsonValue> for Model {
    type Error = serde_json::Error;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}

impl ActiveModelBehavior for ActiveModel {}
