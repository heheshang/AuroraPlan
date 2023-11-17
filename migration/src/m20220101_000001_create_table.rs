use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(
        &self,
        manager: &SchemaManager,
    ) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        // let db = manager.get_database_backend();
        // let statement = match db {
        //     DbBackend::MySql => Statement::from_string(db, ""),
        //     DbBackend::Postgres => Statement::from_string(db, "ALTER TABLE IF EXISTS ONLY public.t_ds_task_instance DROP CONSTRAINT IF EXISTS foreign_key_instance_id;"),
        //     DbBackend::Sqlite => Statement::from_string(db, ""),
        // };
        // manager.get_connection().execute(statement).await?;
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Text).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(
        &self,
        manager: &SchemaManager,
    ) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager.drop_table(Table::drop().table(Post::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
