use sea_orm::{entity::prelude::*, DeleteResult};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32, // 自增id
    #[sea_orm(unique)]
    pub uuid: String, // 用户UUID
    #[sea_orm(unique)]
    pub user_name: String, // 用户名
    pub pass_word: String,           // 密码
    pub email: Option<String>,       // 邮箱
    pub age: Option<i32>,            // 年龄
    pub image: Option<String>,       // 头像
    pub phone: Option<String>,       // 手机号
    pub role: Option<String>,        // 角色
    pub permissions: Option<String>, // 权限
    pub binding: Option<String>,     // authentication绑定
    #[sea_orm(default_value_t = DateTimeUtc::default())]
    pub created_at: DateTimeUtc,
    #[sea_orm(default_value_t = DateTimeUtc::default())]
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Entity {
    // 添加按UUID查询的方法
    pub fn find_by_uuid(uuid: &str) -> Select<Entity> {
        Self::find().filter(Column::Uuid.eq(uuid))
    }

    // 添加使用UUID删除的方法
    pub async fn delete_by_uuid(
        db: &DatabaseConnection,
        uuid: &str,
    ) -> Result<DeleteResult, sea_orm::DbErr> {
        let result = Self::delete_many()
            .filter(Column::Uuid.eq(uuid))
            .exec(db)
            .await?;
        Ok(result)
    }
}

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
