use diesel::{Connection, PgConnection};

pub mod pagination;
pub mod schema;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _conn = &mut PgConnection::establish(&database_url)?;
    println!("Hello, world!");
    Ok(())
}
