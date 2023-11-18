//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "ip_address"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub ip_metadata_id: Option<i64>,
    pub addr: Option<String>,
    pub addr_type: Option<String>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    IpMetadataId,
    Addr,
    AddrType,
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
    IpMetadata,
    IpPort,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::IpMetadataId => ColumnType::BigInteger.def().null(),
            Self::Addr => ColumnType::Text.def().null(),
            Self::AddrType => ColumnType::Text.def().null(),
            Self::CreatedAt => ColumnType::TimestampWithTimeZone.def().null(),
            Self::UpdatedAt => ColumnType::TimestampWithTimeZone.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::IpMetadata => Entity::belongs_to(super::ip_metadata::Entity)
                .from(Column::IpMetadataId)
                .to(super::ip_metadata::Column::Id)
                .into(),
            Self::IpPort => Entity::has_many(super::ip_port::Entity).into(),
        }
    }
}

impl Related<super::ip_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpMetadata.def()
    }
}

impl Related<super::ip_port::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpPort.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
