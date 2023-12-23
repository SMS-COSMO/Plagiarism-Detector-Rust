use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Generate paper table
        manager
            .create_table(
                Table::create()
                    .table(Papers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Papers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Papers::Pid).string().not_null())
                    .col(ColumnDef::new(Papers::Text).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Generate name table
        manager
            .create_table(
                Table::create()
                    .table(Names::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Names::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Names::Name).string().not_null())
                    .col(ColumnDef::new(Names::Df).integer().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Names::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Papers {
    Table,
    Id,
    Pid,
    Text,
}

#[derive(DeriveIden)]
enum Names {
    Table,
    Id,
    Name,
    Df,
}
