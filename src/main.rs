use ntex::web;

mod error;
mod models;
mod services;

#[web::get("/health")]
async fn health_check() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

  web::server(|| {
    web::App::new()
      .wrap(web::middleware::Logger::default())
      .configure(services::openapi::ntex_config)
      .configure(services::todo::ntex_config)
      .service(health_check)
      .default_service(web::route().to(services::default))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())
}
