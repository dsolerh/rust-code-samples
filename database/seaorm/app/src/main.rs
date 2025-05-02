use entities::prelude::*;
use migrations::{Migrator, MigratorTrait};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, ConnectionTrait, Database, EntityTrait,
    PaginatorTrait, QueryFilter, Statement,
};

const DATABASE_URL: &str = concat!(
    "postgres://",
    "test", // db username
    ':',
    "pass", // db password
    '@',
    "localhost", // db host
    ':',
    "5432", // db port
);

// custom database to create within the program
const DB_NAME: &str = "seatest";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let db = {
        let db = Database::connect(&format!("{}/{}", DATABASE_URL, "/testdb")).await?;

        // drop the existing db if any
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
        ))
        .await?;

        // create the database
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("CREATE DATABASE \"{}\"", DB_NAME),
        ))
        .await?;

        let url = format!("{}/{}", DATABASE_URL, DB_NAME);
        Database::connect(&url).await?
    };

    assert!(db.ping().await.is_ok());
    println!("connected");

    // run migrations
    Migrator::refresh(&db).await?;

    Bakery::insert_many([
        entities::bakery::ActiveModel {
            name: Set("Ma baker".to_owned()),
            profit_marging: Set(12.4),
            ..Default::default()
        },
        entities::bakery::ActiveModel {
            name: Set("Costa".to_owned()),
            profit_marging: Set(11.4),
            ..Default::default()
        },
        entities::bakery::ActiveModel {
            name: Set("Starbucks".to_owned()),
            profit_marging: Set(10.4),
            ..Default::default()
        },
    ])
    .exec(&db)
    .await?;

    Bakery::update_many()
        .set(entities::bakery::ActiveModel {
            profit_marging: Set(14.6),
            ..Default::default()
        })
        .filter(
            Condition::any()
                .add(entities::bakery::Column::Name.contains("Ma"))
                .add(entities::bakery::Column::ProfitMarging.lt(11.0)),
        )
        .exec(&db)
        .await?;

    let bakery_pages = Bakery::find()
        .filter(
            Condition::any()
                .add(entities::bakery::Column::Name.contains("Ma"))
                .add(entities::bakery::Column::ProfitMarging.gte(10.4)),
        )
        .paginate(&db, 20);

    let bakeries = bakery_pages.fetch_page(0).await?;
    println!("{bakeries:?}");

    // Closing connection here
    db.close().await?;
    Ok(())
}
