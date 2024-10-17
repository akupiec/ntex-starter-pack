use crate::infra::connect;
use log::info;
use ntex::web;

mod infra;
mod todo;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

  info!("Starting DB & Migrations");
  let conn = connect().await.expect("Failed to connect to the database");

  println!("cargo:rerun-if-changed=migrations");
  sqlx::migrate!("./migrations")
    .run(&conn)
    .await
    .expect("Fail to run migrations");

  web::server(move || {
    web::App::new()
      .wrap(web::middleware::Logger::default())
      .configure(infra::infrastructure_config)
      .state(conn.clone())
      .configure(todo::ntex_config)
      .default_service(web::route().to(todo::default))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())
}
