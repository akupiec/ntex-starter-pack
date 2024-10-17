use sqlx::{Error, SqlitePool};
use std::fs::File;

pub async fn connect() -> Result<SqlitePool, Error> {
  File::create_new("test.sqlite").ok();
  SqlitePool::connect("sqlite://test.sqlite").await
}
