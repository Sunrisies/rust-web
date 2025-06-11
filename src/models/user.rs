use sea_orm::{entity::prelude::*, DeleteResult};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uuid: String,
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

impl ActiveModelBehavior for ActiveModel {}
