use std::sync::Arc;

use anyhow::Ok;
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};
use sqlx::{PgPool, Row};

fn main() {}

#[allow(dead_code)]
async fn handle_add_todo(repo: impl TodoRepo, description: String) -> anyhow::Result<()> {
    repo.add_todo(description).await?;
    Ok(())
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TodoRepo {
    async fn add_todo(&self, description: String) -> anyhow::Result<i64>;
    // fn add_todo(&self, description: String) -> impl Future<Output = anyhow::Result<i64>> + Send;
}

#[allow(dead_code)]
struct TodoItem {
    description: String,
    done: bool,
}

#[allow(dead_code)]
struct PgTodoRepo {
    pg_pool: Arc<PgPool>,
}

#[allow(dead_code)]
const ADD_TODO_QUERY: &str = r#"
INSERT INTO todos ( description )
VALUES ( $1 )
RETURNING id"#;

#[async_trait]
impl TodoRepo for PgTodoRepo {
    async fn add_todo(&self, description: String) -> anyhow::Result<i64> {
        let req = sqlx::query(ADD_TODO_QUERY)
            .bind(description)
            .fetch_one(&*self.pg_pool)
            .await?;
        Ok(req.get("id"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mocked_add() {
        let description = String::from("value");
        let mut todo_repo = MockTodoRepo::new();
        todo_repo
            .expect_add_todo()
            .times(1)
            .with(eq(description.clone()))
            .returning(|_| Ok(1 as i64));

        handle_add_todo(todo_repo, description).await.unwrap();
    }
}
