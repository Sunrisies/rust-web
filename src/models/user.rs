use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid; // 添加 uuid 依赖
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    // 主键
    pub id: i32,
    // 全局唯一标识符
    #[sea_orm(unique)]
    pub uuid: Uuid,
    // 用户名
    pub username: String,
    // 电子邮箱
    #[sea_orm(unique)]
    pub email: String,
    // 密码
    pub password: String,
    // 年龄
    pub age: Option<i32>,
    // 创建时间
    pub created_at: DateTimeUtc,
    // 更新时间
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
