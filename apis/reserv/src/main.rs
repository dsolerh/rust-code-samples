use anyhow::Context;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, sqlx::FromRow)]
struct TablesInfo {
    table_id: i32,
    capacity: i16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .context("context")?;

    sqlx::migrate!("./migrations").run(&db).await?;

    let res = sqlx::query(
        r#"
        insert into tables_info (capacity)
        values (2),(2),(4),(4),(6),(6),(8),(8),(10);
    "#,
    )
    .execute(&db)
    .await?;

    println!("rows affected {}", res.rows_affected());

    let tables_info: Vec<TablesInfo> = sqlx::query_as("select * from tables_info")
        .fetch_all(&db)
        .await?;

    println!("tables_info: {tables_info:?}");

    Ok(())
}
