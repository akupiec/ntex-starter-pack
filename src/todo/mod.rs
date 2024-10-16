mod model;
mod todo;

use ntex::web;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
  todo::get_todos,
  todo::create_todo,
  todo::delete_todo,
  todo::get_todo,
  todo::update_todo
))]
pub struct TodoApi;

pub async fn default() -> web::HttpResponse {
  web::HttpResponse::NotFound().finish()
}

pub fn ntex_config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/todos")
      .service(todo::get_todos)
      .service(todo::create_todo)
      .service(todo::get_todo)
      .service(todo::update_todo)
      .service(todo::delete_todo),
  );
}
