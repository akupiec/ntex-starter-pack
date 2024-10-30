use log::info;
use sqlx::SqlitePool;
use std::fs::File;

pub async fn db_pool() -> SqlitePool {
  info!("Starting DB & Migrations");
  File::create_new("test.sqlite").ok();
  let pool = SqlitePool::connect("sqlite://test.sqlite")
    .await
    .expect("Failed to connect to the database");

  println!("cargo:rerun-if-changed=migrations");
  sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .expect("Fail to run migrations");

  pool
}
