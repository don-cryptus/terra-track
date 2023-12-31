pub use sea_orm_migration::prelude::*;
pub mod db;

mod m_000001_scan_batch;
mod m_000002_ip_main;
mod m_000003_ip_location;
mod m_000004_ip_network_details;
mod m_000005_ip_security_flags;
mod m_000006_ip_organization;
mod m_000007_ip_flag;
mod m_000008_ip_hosting_details;
mod m_000009_ip_os;
mod m_000010_ip_host_script;
mod m_000011_ip_service;
mod m_000012_ip_service_script;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m_000001_scan_batch::Migration),
            Box::new(m_000002_ip_main::Migration),
            Box::new(m_000003_ip_location::Migration),
            Box::new(m_000004_ip_network_details::Migration),
            Box::new(m_000005_ip_security_flags::Migration),
            Box::new(m_000006_ip_organization::Migration),
            Box::new(m_000007_ip_flag::Migration),
            Box::new(m_000008_ip_hosting_details::Migration),
            Box::new(m_000009_ip_os::Migration),
            Box::new(m_000010_ip_host_script::Migration),
            Box::new(m_000011_ip_service::Migration),
            Box::new(m_000012_ip_service_script::Migration),
        ]
    }
}
