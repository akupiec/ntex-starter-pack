mod common;
mod infra;
mod user;

use crate::infra::db_pool;
use crate::user::user_controller;
use ntex::web;

#[web::get("")]
pub async fn default() -> web::HttpResponse {
  web::HttpResponse::NotFound().finish()
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

  let pool = db_pool().await;

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
