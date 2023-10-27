use std::env;

// use futures::executor::block_on;
use async_std::{fs, task};
use sea_orm_migration::{
    prelude::*,
    sea_orm::{Database, Statement},
};
fn main() -> Result<(), DbErr> {
    if let Err(err) = task::block_on(run()) {
        panic!("{}", err);
    }

    // cli::run_cli(migration::Migrator).await;

    Ok(())
}
async fn run() -> Result<(), DbErr> {
    let db_url = dotenvy::var("DATABASE_URL")
        .unwrap_or(DbErr::Custom("DATABASE_URL is not set".to_string()).to_string());
    println!("db_url: {:?}", db_url);

    let current_dir = env::current_dir().unwrap_or(
        DbErr::Custom("current_dir is not set".to_string())
            .to_string()
            .into(),
    );
    println!("current_dir: {:?}", current_dir);

    let init_sql_path = current_dir
        .to_str()
        .unwrap_or(
            DbErr::Custom("current_dir is not set".to_string())
                .to_string()
                .as_str(),
        )
        .split("migration")
        .collect::<Vec<_>>()
        .first()
        .unwrap_or(
            &DbErr::Custom("current_dir is not set".to_string())
                .to_string()
                .as_str(),
        )
        .to_string()
        + "/migration/init.sql";

    let sql_content = fs::read_to_string(init_sql_path)
        .await
        .expect("read init.sql error")
        .lines()
        .filter(|ll| !ll.starts_with("--") && !ll.is_empty())
        .map(|ll| ll.trim())
        .collect::<Vec<_>>()
        .join(" ");
    println!("sql_content: {:?}", sql_content);

    let db = Database::connect(&db_url).await?;
    match db.get_database_backend() {
        sea_orm::DatabaseBackend::MySql => {
            db.execute(Statement::from_string(db.get_database_backend(), ""))
                .await?;
        }
        sea_orm::DatabaseBackend::Postgres => {
            for l in sql_content.split(';') {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("{};", l),
                ))
                .await?;
                println!("sql : {:?} exec success", l);
            }
        }
        sea_orm::DatabaseBackend::Sqlite => {
            db.execute(Statement::from_string(db.get_database_backend(), ""))
                .await?;
        }
    };

    cli::run_cli(migration::Migrator).await;

    Ok(())
}
