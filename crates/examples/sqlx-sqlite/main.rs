use anyhow::Ok;
use clap::{Parser, Subcommand};
use sqlx::{Row, SqlitePool};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Add { description: String },
    Done { id: i64 },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    dotenvy::from_filename("examples/sqlx-sqlite/.env")?;
    println!("Hello from sqlx!!");

    let database_url = dotenvy::var("DATABASE_URL")?;
    println!("DATABASE_URL={}", database_url);

    let pool = SqlitePool::connect(&database_url).await?;
    match args.cmd {
        Some(Command::Add { description }) => {
            println!("Adding new todo with description '{description}'");
            let todo_id = add_todo(&pool, description).await?;
            println!("Added new todo with id {todo_id}");
        }
        Some(Command::Done { id }) => {
            println!("Marking todo {id} as done");
            if complete_todo(&pool, id).await? {
                println!("Todo {id} is marked as done");
            } else {
                println!("Invalid id {id}");
            }
        }
        None => {
            let todos = get_all_todos(&pool).await?;
            for (id, description, done) in todos {
                println!(
                    "- [{}] {}: {}",
                    if done { "x" } else { " " },
                    id,
                    description,
                );
            }
        }
    }

    Ok(())
}

const ADD_TODO_QUERY: &str = r#"
INSERT INTO todos ( description )
VALUES ( ?1 )
"#;

async fn add_todo(pool: &SqlitePool, description: String) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;

    let id = sqlx::query(ADD_TODO_QUERY)
        .bind(description)
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

    Ok(id)
}

const COMPLETE_TODO_QUERY: &str = r#"
UPDATE todos
SET done = TRUE
WHERE id = ?1
"#;

async fn complete_todo(pool: &SqlitePool, id: i64) -> anyhow::Result<bool> {
    let affected = sqlx::query(COMPLETE_TODO_QUERY)
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(affected > 0)
}

const GET_ALL_TODOS_QUERY: &str = r#"
SELECT id, description, done
FROM todos
ORDER BY id
LIMIT 10
"#;

async fn get_all_todos(pool: &SqlitePool) -> anyhow::Result<Vec<(i64, String, bool)>> {
    Ok(sqlx::query(GET_ALL_TODOS_QUERY)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|x| (x.get("id"), x.get("description"), x.get("done")))
        .collect::<Vec<(i64, String, bool)>>())
}
