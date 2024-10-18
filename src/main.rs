use crate::infra::db_pool;
use crate::user::user_controller;
use log::info;
use ntex::web;

mod common;
mod infra;
mod user;

#[web::get("")]
pub async fn default() -> web::HttpResponse {
  web::HttpResponse::NotFound().finish()
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

  info!("Starting DB & Migrations");
  let pool = db_pool().await.expect("Failed to connect to the database");

  println!("cargo:rerun-if-changed=migrations");
  sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .expect("Fail to run migrations");

  web::server(move || {
    web::App::new()
      .wrap(web::middleware::Logger::default())
      .configure(infra::infrastructure_config)
      .state(pool.clone())
      .configure(user_controller)
      .service(default)
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())
}
