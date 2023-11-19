//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use super::sea_orm_active_enums::Protocol;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "ip_service"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub ip_main_id: i64,
    pub protocol: Protocol,
    pub port: i16,
    pub name: String,
    pub product: Option<String>,
    pub service_fp: Option<String>,
    pub version: Option<String>,
    pub extra_info: Option<String>,
    pub os_type: Option<String>,
    pub cpu_arch: Option<String>,
    pub method: Option<String>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    IpMainId,
    Protocol,
    Port,
    Name,
    Product,
    ServiceFp,
    Version,
    ExtraInfo,
    OsType,
    CpuArch,
    Method,
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
            Self::IpMainId => ColumnType::BigInteger.def(),
            Self::Protocol => Protocol::db_type().def(),
            Self::Port => ColumnType::SmallInteger.def(),
            Self::Name => ColumnType::String(None).def(),
            Self::Product => ColumnType::String(None).def().null(),
            Self::ServiceFp => ColumnType::Text.def().null(),
            Self::Version => ColumnType::String(None).def().null(),
            Self::ExtraInfo => ColumnType::String(None).def().null(),
            Self::OsType => ColumnType::String(None).def().null(),
            Self::CpuArch => ColumnType::String(None).def().null(),
            Self::Method => ColumnType::String(None).def().null(),
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
