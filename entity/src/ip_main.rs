//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "ip_main"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub ip_type: String,
    pub registry: String,
    pub ip_address: String,
    pub created_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    IpType,
    Registry,
    IpAddress,
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
    IpFlag,
    IpHostScript,
    IpHostingDetails,
    IpLocation,
    IpNetworkDetails,
    IpOrganization,
    IpOs,
    IpSecurityFlags,
    IpService,
    IpServiceScript,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::IpType => ColumnType::String(None).def(),
            Self::Registry => ColumnType::String(None).def(),
            Self::IpAddress => ColumnType::Text.def().unique(),
            Self::CreatedAt => ColumnType::TimestampWithTimeZone.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::IpFlag => Entity::has_many(super::ip_flag::Entity).into(),
            Self::IpHostScript => Entity::has_many(super::ip_host_script::Entity).into(),
            Self::IpHostingDetails => Entity::has_many(super::ip_hosting_details::Entity).into(),
            Self::IpLocation => Entity::has_many(super::ip_location::Entity).into(),
            Self::IpNetworkDetails => Entity::has_many(super::ip_network_details::Entity).into(),
            Self::IpOrganization => Entity::has_many(super::ip_organization::Entity).into(),
            Self::IpOs => Entity::has_many(super::ip_os::Entity).into(),
            Self::IpSecurityFlags => Entity::has_many(super::ip_security_flags::Entity).into(),
            Self::IpService => Entity::has_many(super::ip_service::Entity).into(),
            Self::IpServiceScript => Entity::has_many(super::ip_service_script::Entity).into(),
        }
    }
}

impl Related<super::ip_flag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpFlag.def()
    }
}

impl Related<super::ip_host_script::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpHostScript.def()
    }
}

impl Related<super::ip_hosting_details::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpHostingDetails.def()
    }
}

impl Related<super::ip_location::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpLocation.def()
    }
}

impl Related<super::ip_network_details::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpNetworkDetails.def()
    }
}

impl Related<super::ip_organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpOrganization.def()
    }
}

impl Related<super::ip_os::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpOs.def()
    }
}

impl Related<super::ip_security_flags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpSecurityFlags.def()
    }
}

impl Related<super::ip_service::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpService.def()
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
    #[sea_orm(entity = "super::ip_flag::Entity")]
    IpFlag,
    #[sea_orm(entity = "super::ip_host_script::Entity")]
    IpHostScript,
    #[sea_orm(entity = "super::ip_hosting_details::Entity")]
    IpHostingDetails,
    #[sea_orm(entity = "super::ip_location::Entity")]
    IpLocation,
    #[sea_orm(entity = "super::ip_network_details::Entity")]
    IpNetworkDetails,
    #[sea_orm(entity = "super::ip_organization::Entity")]
    IpOrganization,
    #[sea_orm(entity = "super::ip_os::Entity")]
    IpOs,
    #[sea_orm(entity = "super::ip_security_flags::Entity")]
    IpSecurityFlags,
    #[sea_orm(entity = "super::ip_service::Entity")]
    IpService,
    #[sea_orm(entity = "super::ip_service_script::Entity")]
    IpServiceScript,
}
