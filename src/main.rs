use ntex::web;

mod infra;
mod todo;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

  web::server(|| {
    web::App::new()
      .wrap(web::middleware::Logger::default())
      .configure(infra::infrastructure_config)
      .configure(todo::ntex_config)
      .default_service(web::route().to(todo::default))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())
}
