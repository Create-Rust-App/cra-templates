//! Todo CRUD routes backed by SQLite.
//!
//! Registered on [`crate::app::CUSTOM_ROUTERS`] so the routes merge under
//! the API prefix without forking the template router.

use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};

use crate::{
    app::{AppState, CUSTOM_ROUTERS},
    todos::{self, NewTodo, Todo},
};

/// Router for the todo feature; nested under the API prefix by the app.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/todos", get(list).post(create))
        .route("/todos/:id", get(get_one).delete(delete_one))
}

/// List all todos.
async fn list() -> Result<Json<Vec<Todo>>, StatusCode> {
    todos::list_todos()
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Create a todo, returning it with its assigned id.
async fn create(Json(payload): Json<NewTodo>) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    if payload.title.trim().is_empty() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    todos::create_todo(&payload.title)
        .await
        .map(|todo| (StatusCode::CREATED, Json(todo)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Fetch a todo by id.
async fn get_one(Path(id): Path<i64>) -> Result<Json<Todo>, StatusCode> {
    todos::get_todo(id)
        .await
        .map(Json)
        .map_err(|error| match error {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

/// Delete a todo by id.
async fn delete_one(Path(id): Path<i64>) -> Result<StatusCode, StatusCode> {
    // Confirm the row exists so unknown ids report 404 instead of 204.
    todos::get_todo(id).await.map_err(|error| match error {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;
    todos::delete_todo(id)
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[linkme::distributed_slice(CUSTOM_ROUTERS)]
static TODO_ROUTER: fn() -> Router<AppState> = router;
