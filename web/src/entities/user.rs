//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(column_type = "Text")]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::email_address::Entity")]
    EmailAddress,
    #[sea_orm(has_many = "super::provider_connection::Entity")]
    ProviderConnection,
}

impl Related<super::email_address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EmailAddress.def()
    }
}

impl Related<super::provider_connection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProviderConnection.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
