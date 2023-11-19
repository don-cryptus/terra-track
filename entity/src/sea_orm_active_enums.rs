//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "protocol")]
pub enum Protocol {
    #[sea_orm(string_value = "TCP")]
    Tcp,
    #[sea_orm(string_value = "UPD")]
    Upd,
}