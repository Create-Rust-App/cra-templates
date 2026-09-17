//! Todo persistence over the shared [`crate::db`] pool.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::db::pool;

/// A todo row.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, Eq)]
pub struct Todo {
    /// Row id assigned by SQLite.
    pub id: i64,
    /// Short description of the task.
    pub title: String,
    /// Completion flag.
    pub done: bool,
}

/// Payload for creating a todo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTodo {
    /// Short description of the task.
    pub title: String,
}

/// Insert a todo and return it with its assigned id.
pub async fn create_todo(title: &str) -> Result<Todo, sqlx::Error> {
    sqlx::query_as::<_, Todo>("INSERT INTO todos (title) VALUES (?) RETURNING id, title, done")
        .bind(title)
        .fetch_one(pool().await)
        .await
}

/// Fetch a todo by id, or [`sqlx::Error::RowNotFound`].
pub async fn get_todo(id: i64) -> Result<Todo, sqlx::Error> {
    sqlx::query_as::<_, Todo>("SELECT id, title, done FROM todos WHERE id = ?")
        .bind(id)
        .fetch_one(pool().await)
        .await
}

/// List todos in id order.
pub async fn list_todos() -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as::<_, Todo>("SELECT id, title, done FROM todos ORDER BY id")
        .fetch_all(pool().await)
        .await
}

/// Delete a todo by id. Succeeds even when the row does not exist.
pub async fn delete_todo(id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(pool().await)
        .await
        .map(|_| ())
}
