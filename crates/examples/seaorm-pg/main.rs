use sea_orm::{ConnectOptions, Database};
use std::time::Duration;

mod migrator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename("examples/seaorm-pg/.env")?;
    let database_url = dotenvy::var("DATABASE_URL")?;
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    // .sqlx_logging_level(log::LevelFilter::Info)
    // .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await?;

    Ok(())
}
