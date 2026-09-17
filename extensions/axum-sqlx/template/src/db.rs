//! Shared SQLite pool plus embedded migrations.
//!
//! The pool is created on first use from `DATABASE_URL` (defaults to a file
//! database under `./data`) and migrations in `./migrations` run exactly
//! once per process.

use std::{path::Path, sync::OnceLock};

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

/// Shared pool, initialised on first use.
static POOL: OnceLock<SqlitePool> = OnceLock::new();

/// Database URL. Production projects point this at a file (or `libsql`/Turso
/// URL via a different driver); tests use `sqlite::memory:`.
fn database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:data/app.db?mode=rwc".to_string())
}

/// Create the parent directory of file-backed database URLs, since SQLite
/// creates the file but not its parents.
fn ensure_parent_dir(url: &str) {
    let path = url.strip_prefix("sqlite:").unwrap_or(url);
    let path = path.split('?').next().unwrap_or(path);
    if path == ":memory:" {
        return;
    }
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("create database directory");
        }
    }
}

/// Shared, migrated pool for the whole service.
pub async fn pool() -> &'static SqlitePool {
    if let Some(pool) = POOL.get() {
        return pool;
    }
    let url = database_url();
    ensure_parent_dir(&url);
    let mut options = SqlitePoolOptions::new().max_connections(5);
    if url.contains(":memory:") {
        // In-memory SQLite databases are scoped to a single connection.
        options = options.max_connections(1);
    }
    let pool = options
        .connect(&url)
        .await
        .expect("connect to the database");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("run database migrations");
    POOL.get_or_init(|| pool)
}
