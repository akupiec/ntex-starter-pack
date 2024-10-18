mod database;
mod health;
pub mod model;
mod openapi;

pub use database::db_pool;

use crate::user;
use ntex::web;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        description = "DESCRIPTION",
    ),
    paths(health::health_check),
    nest(
      (path = "/users", api = user::UserApi)
    )
)]
struct ApiDoc;

pub fn infrastructure_config(config: &mut web::ServiceConfig) {
  config.service(openapi::ntex_service());
  config.service(health::health_check);
}
