//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

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
    pub protocol: String,
    pub port: i32,
    pub name: String,
    pub conf: i16,
    pub version: Option<String>,
    pub product: Option<String>,
    pub extra_info: Option<String>,
    pub tunnel: Option<String>,
    pub proto: Option<String>,
    pub rpcnum: Option<String>,
    pub lowver: Option<String>,
    pub highver: Option<String>,
    pub hostname: Option<String>,
    pub method: String,
    pub os_type: Option<String>,
    pub cpu_arch: Option<String>,
    pub device_type: Option<String>,
    pub service_fp: Option<String>,
    pub cpe: Option<String>,
    pub created_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    IpMainId,
    Protocol,
    Port,
    Name,
    Conf,
    Version,
    Product,
    ExtraInfo,
    Tunnel,
    Proto,
    Rpcnum,
    Lowver,
    Highver,
    Hostname,
    Method,
    OsType,
    CpuArch,
    DeviceType,
    ServiceFp,
    Cpe,
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
    IpServiceScript,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::IpMainId => ColumnType::BigInteger.def(),
            Self::Protocol => ColumnType::String(None).def(),
            Self::Port => ColumnType::Integer.def(),
            Self::Name => ColumnType::String(None).def(),
            Self::Conf => ColumnType::SmallInteger.def(),
            Self::Version => ColumnType::String(None).def().null(),
            Self::Product => ColumnType::String(None).def().null(),
            Self::ExtraInfo => ColumnType::String(None).def().null(),
            Self::Tunnel => ColumnType::String(None).def().null(),
            Self::Proto => ColumnType::String(None).def().null(),
            Self::Rpcnum => ColumnType::String(None).def().null(),
            Self::Lowver => ColumnType::String(None).def().null(),
            Self::Highver => ColumnType::String(None).def().null(),
            Self::Hostname => ColumnType::String(None).def().null(),
            Self::Method => ColumnType::String(None).def(),
            Self::OsType => ColumnType::String(None).def().null(),
            Self::CpuArch => ColumnType::String(None).def().null(),
            Self::DeviceType => ColumnType::String(None).def().null(),
            Self::ServiceFp => ColumnType::String(None).def().null(),
            Self::Cpe => ColumnType::String(None).def().null(),
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
            Self::IpServiceScript => Entity::has_many(super::ip_service_script::Entity).into(),
        }
    }
}

impl Related<super::ip_main::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpMain.def()
    }
}

impl Related<super::ip_service_script::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpServiceScript.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::ip_main::Entity")]
    IpMain,
    #[sea_orm(entity = "super::ip_service_script::Entity")]
    IpServiceScript,
}
