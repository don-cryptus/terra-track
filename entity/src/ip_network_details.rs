//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "ip_network_details"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub ip_main_id: Option<i64>,
    pub ptr_record: Option<String>,
    pub asn_number: Option<i64>,
    pub asn_name: Option<String>,
    pub ip_range: Option<String>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    IpMainId,
    PtrRecord,
    AsnNumber,
    AsnName,
    IpRange,
    CreatedAt,
    UpdatedAt,
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
            Self::PtrRecord => ColumnType::Text.def().null(),
            Self::AsnNumber => ColumnType::BigInteger.def().null(),
            Self::AsnName => ColumnType::Text.def().null(),
            Self::IpRange => ColumnType::Text.def().null(),
            Self::CreatedAt => ColumnType::TimestampWithTimeZone.def().null(),
            Self::UpdatedAt => ColumnType::TimestampWithTimeZone.def().null(),
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
