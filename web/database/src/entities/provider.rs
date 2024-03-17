//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::ProviderType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "provider")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub name: String,
    pub slug: Option<String>,
    pub r#type: ProviderType,
    #[sea_orm(column_type = "Text", nullable)]
    pub client_id: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub client_secret: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::provider_connection::Entity")]
    ProviderConnection,
}

impl Related<super::provider_connection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProviderConnection.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
