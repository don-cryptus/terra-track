use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ScanBatch::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ScanBatch::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ScanBatch::Ip).string().not_null())
                    .col(ColumnDef::new(ScanBatch::Cursor).integer().not_null())
                    .col(ColumnDef::new(ScanBatch::Size).integer().not_null())
                    .col(
                        ColumnDef::new(ScanBatch::Start)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ScanBatch::End)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(ScanBatch::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(ScanBatch::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ScanBatch::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ScanBatch {
    Table,
    Id,
    Ip,
    Cursor,
    Size,
    Start,
    End,
    CreatedAt,
    UpdatedAt,
}
