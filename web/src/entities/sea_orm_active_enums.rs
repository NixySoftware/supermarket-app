//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "provider_type")]
pub enum ProviderType {
    #[sea_orm(string_value = "apple")]
    Apple,
    #[sea_orm(string_value = "google")]
    Google,
    #[sea_orm(string_value = "microsoft")]
    Microsoft,
}
