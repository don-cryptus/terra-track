//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

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
    pub id: i32,
    pub ip_address: String,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    IpAddress,
    CreatedAt,
    UpdatedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    IpAbuseContact,
    IpConnection,
    IpContactDetails,
    IpFlag,
    IpHostingDetails,
    IpLocation,
    IpMetadata,
    IpNetworkDetails,
    IpOrganization,
    IpPrivacy,
    IpSecurityFlags,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::IpAddress => ColumnType::Text.def().unique(),
            Self::CreatedAt => ColumnType::TimestampWithTimeZone.def().null(),
            Self::UpdatedAt => ColumnType::TimestampWithTimeZone.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::IpAbuseContact => Entity::has_many(super::ip_abuse_contact::Entity).into(),
            Self::IpConnection => Entity::has_many(super::ip_connection::Entity).into(),
            Self::IpContactDetails => Entity::has_many(super::ip_contact_details::Entity).into(),
            Self::IpFlag => Entity::has_many(super::ip_flag::Entity).into(),
            Self::IpHostingDetails => Entity::has_many(super::ip_hosting_details::Entity).into(),
            Self::IpLocation => Entity::has_many(super::ip_location::Entity).into(),
            Self::IpMetadata => Entity::has_many(super::ip_metadata::Entity).into(),
            Self::IpNetworkDetails => Entity::has_many(super::ip_network_details::Entity).into(),
            Self::IpOrganization => Entity::has_many(super::ip_organization::Entity).into(),
            Self::IpPrivacy => Entity::has_many(super::ip_privacy::Entity).into(),
            Self::IpSecurityFlags => Entity::has_many(super::ip_security_flags::Entity).into(),
        }
    }
}

impl Related<super::ip_abuse_contact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpAbuseContact.def()
    }
}

impl Related<super::ip_connection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpConnection.def()
    }
}

impl Related<super::ip_contact_details::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpContactDetails.def()
    }
}

impl Related<super::ip_flag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpFlag.def()
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

impl Related<super::ip_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpMetadata.def()
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

impl Related<super::ip_privacy::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpPrivacy.def()
    }
}

impl Related<super::ip_security_flags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IpSecurityFlags.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
