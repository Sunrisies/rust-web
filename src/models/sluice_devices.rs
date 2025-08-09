use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sluice_devices")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u32,
    #[sea_orm(column_name = "subscribe_topic", unique)]
    pub subscribe_topic: String,
    #[sea_orm(column_name = "publish_topic")]
    pub publish_topic: String,
    pub create_time: i64,
    #[sea_orm(column_name = "remark", nullable)]
    pub remark: Option<String>,
    #[sea_orm(column_name = "device_name")]
    pub device_name: String,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sluice_command::Entity")]
    SluiceCommand,
}
impl Related<super::sluice_command::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SluiceCommand.def()
    }
}
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
