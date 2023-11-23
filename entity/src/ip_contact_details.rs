//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "ip_contact_details"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub ip_main_id: Option<i64>,
    pub contact_name: Option<String>,
    pub contact_address: Option<String>,
    pub phone: Option<String>,
    pub remarks: Option<String>,
    pub created_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    IpMainId,
    ContactName,
    ContactAddress,
    Phone,
    Remarks,
    CreatedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    IpMain,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::IpMainId => ColumnType::BigInteger.def().null(),
            Self::ContactName => ColumnType::Text.def().null(),
            Self::ContactAddress => ColumnType::Text.def().null(),
            Self::Phone => ColumnType::Text.def().null(),
            Self::Remarks => ColumnType::Text.def().null(),
            Self::CreatedAt => ColumnType::TimestampWithTimeZone.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::IpMain => Entity::belongs_to(super::ip_main::Entity)
                .from(Column::IpMainId)
                .to(super::ip_main::Column::Id)
                .into(),
        }
    }
}

impl Related<super::ip_main::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpMain.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
