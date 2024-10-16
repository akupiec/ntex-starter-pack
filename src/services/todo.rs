use ntex::http::Response;
use ntex::web;
use crate::error::HttpError;
use crate::models::todo::Todo;
use crate::models::todo::TodoPartial;

#[utoipa::path(
  get,
  path = "/todos", 
  responses(
    (status = 200, description = "List of Todo", body = [Todo])
))]
#[web::get("/todos")]
pub async fn get_todos() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

/// Create a new todo
#[utoipa::path(
  post,
  path = "/todos",
  request_body = TodoPartial,
  responses(
    (status = 201, description = "Todo created", body = Todo),
  ),
)]
#[web::post("/todos")]
pub async fn create_todo(_todo: web::types::Json<TodoPartial>) -> web::HttpResponse {
  web::HttpResponse::Created().finish()
}

/// Get a todo by id
#[utoipa::path(
  get,
  path = "/todos/{id}",
  responses(
    (status = 200, description = "Todo found", body = Todo),
    (status = 400, description = "Todo not found", body = HttpError),
  ),
)]
#[web::get("/todos/{id}")]
pub async fn get_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

/// Update a todo by id
#[utoipa::path(
  put,
  path = "/todos/{id}",
  request_body = TodoPartial,
  responses(
    (status = 200, description = "Todo updated", body = Todo),
    (status = 400, description = "Todo not found", body = HttpError),
  ),
)]
#[web::put("/todos/{id}")]
pub async fn update_todo(path: web::types::Path<u32>) -> Result<Response, HttpError> {
  let id = path.into_inner();
  do_thing_that_fails(id).map_err(|_e| HttpError::ValidationError {field: "test error"})?;
  Ok(web::HttpResponse::Ok().body("OK Reposne"))
}

fn do_thing_that_fails(operation_successful: u32) -> Result<(), &'static str> {
  if operation_successful % 2 == 0 {
    Ok(())
  } else {
    Err("Something went wrong")
  }
}

/// Delete a todo by id
#[utoipa::path(
  delete,
  path = "/todos/{id}",
  responses(
    (status = 200, description = "Todo deleted", body = Todo),
    (status = 404, description = "Todo not found", body = HttpError),
  ),
)]
#[web::delete("/todos/{id}")]
pub async fn delete_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

pub fn ntex_config(cfg: &mut web::ServiceConfig) {
  cfg.service(get_todos);
  cfg.service(create_todo);
  cfg.service(get_todo);
  cfg.service(update_todo);
  cfg.service(delete_todo);
}
