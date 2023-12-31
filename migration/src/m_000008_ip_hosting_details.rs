use crate::m_000002_ip_main::IpMain;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IpHostingDetails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpHostingDetails::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpHostingDetails::IpMainId).big_integer())
                    .col(ColumnDef::new(IpHostingDetails::NumDomains).big_integer())
                    .col(ColumnDef::new(IpHostingDetails::NumMailServers).big_integer())
                    .col(ColumnDef::new(IpHostingDetails::NumNameServers).big_integer())
                    .col(
                        ColumnDef::new(IpHostingDetails::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_hosting_details_ip_main")
                            .from(IpHostingDetails::Table, IpHostingDetails::IpMainId)
                            .to(IpMain::Table, IpMain::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IpHostingDetails::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpHostingDetails {
    Table,
    Id,
    IpMainId,
    NumDomains,
    NumMailServers,
    NumNameServers,
    CreatedAt,
}
