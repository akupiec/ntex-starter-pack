use crate::infra::model::HttpError;
use crate::todo::model::{Todo, TodoPartial};
use ntex::http::Response;
use ntex::web;

#[utoipa::path(
  get,
  path = "/",
  responses(
    (status = 200, description = "List of Todo", body = [Todo])
))]
#[web::get("/")]
pub async fn get_todos() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[utoipa::path(
  post,
  path = "/",
  request_body = TodoPartial,
  responses(
    (status = 201, description = "Todo created", body = Todo),
  ),
)]
#[web::post("/")]
pub async fn create_todo(_todo: web::types::Json<TodoPartial>) -> web::HttpResponse {
  web::HttpResponse::Created().finish()
}

#[utoipa::path(
  get,
  path = "/{id}",
  responses(
    (status = 200, description = "Todo found", body = Todo),
    (status = 400, description = "Todo not found", body = HttpError),
  ),
)]
#[web::get("/{id}")]
pub async fn get_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[utoipa::path(
  put,
  path = "/{id}",
  request_body = TodoPartial,
  responses(
    (status = 200, description = "Todo updated", body = Todo),
    (status = 400, description = "Todo not found", body = HttpError),
  ),
)]
#[web::put("/{id}")]
pub async fn update_todo(path: web::types::Path<u32>) -> Result<Response, HttpError> {
  let id = path.into_inner();
  do_thing_that_fails(id).map_err(|_e| HttpError::ValidationError { field: "test error" })?;
  Ok(web::HttpResponse::Ok().body("OK Reposne"))
}

fn do_thing_that_fails(operation_successful: u32) -> Result<(), &'static str> {
  if operation_successful % 2 == 0 {
    Ok(())
  } else {
    Err("Something went wrong")
  }
}

#[utoipa::path(
  delete,
  path = "/{id}",
  responses(
    (status = 200, description = "Todo deleted", body = Todo),
    (status = 404, description = "Todo not found", body = HttpError),
  ),
)]
#[web::delete("/{id}")]
pub async fn delete_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}
