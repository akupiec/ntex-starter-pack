mod database;
mod health;
pub mod model;
mod openapi;

pub use database::connect;

use crate::todo;
use ntex::web;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        description = "DESCRIPTION",
    ),
    paths(health::health_check),
    nest(
      (path = "/todos", api = todo::TodoApi)
    )
)]
struct ApiDoc;

pub fn infrastructure_config(config: &mut web::ServiceConfig) {
  config.service(openapi::ntex_service());
  config.service(health::health_check);
}
