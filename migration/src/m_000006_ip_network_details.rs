use sea_orm_migration::prelude::*;

use crate::m_000002_ip_main::IpMain;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IpNetworkDetails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpNetworkDetails::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpNetworkDetails::IpMainId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_network_details_ip_main")
                            .from(IpNetworkDetails::Table, IpNetworkDetails::IpMainId)
                            .to(IpMain::Table, IpMain::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(IpNetworkDetails::PtrRecord).text())
                    .col(ColumnDef::new(IpNetworkDetails::AsnNumber).integer())
                    .col(ColumnDef::new(IpNetworkDetails::AsnName).text())
                    .col(ColumnDef::new(IpNetworkDetails::IpRange).text())
                    .col(
                        ColumnDef::new(IpNetworkDetails::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpNetworkDetails::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IpNetworkDetails::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpNetworkDetails {
    Table,
    Id,
    IpMainId,
    PtrRecord,
    AsnNumber,
    AsnName,
    IpRange,
    CreatedAt,
    UpdatedAt,
}
