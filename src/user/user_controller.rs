use crate::infra::model::HttpError;
use crate::response_json;
use crate::user::model::{User, UserUpdate};
use crate::user::user_service as service;
use ntex::http::Response;
use ntex::web;
use sqlx::SqlitePool;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(get_all, update, get, create, delete,))]
pub struct UserApi;

pub fn controller(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/users")
      .service(get_all)
      .service(update)
      .service(get)
      .service(create)
      .service(delete),
  );
}

#[utoipa::path(
  get,
  path = "/",
  responses(
    (status = 200, description = "List of User", body = [User])
))]
#[web::get("/")]
async fn get_all(db: web::types::State<SqlitePool>) -> Result<Response, HttpError> {
  response_json!(service::find_all(db.get_ref()))
}

#[utoipa::path(
  post,
  path = "/",
  request_body = UserUpdate,
  responses(
    (status = 201, description = "User created"),
  ),
)]
#[web::post("/")]
async fn create(db: web::types::State<SqlitePool>, user: web::types::Json<UserUpdate>) -> Result<Response, HttpError> {
  let user = user.into_inner();
  response_json!(service::save(db.get_ref(), user))?;
  Ok(web::HttpResponse::Created().finish())
}

#[utoipa::path(
  get,
  path = "/{id}",
  params(("id" = u32, Path),),
  responses(
    (status = 200, description = "User found", body = User),
    (status = 400, description = "User not found", body = HttpError),
  ),
)]
#[web::get("/{id}")]
async fn get(db: web::types::State<SqlitePool>, path: web::types::Path<u32>) -> Result<Response, HttpError> {
  response_json!(service::find(db.get_ref(), path.into_inner()))
}

#[utoipa::path(
  put,
  path = "/{id}",
  params(("id" = u32, Path),),
  request_body = UserUpdate,
  responses(
    (status = 200, description = "User updated"),
    (status = 400, description = "User not found", body = HttpError),
  ),
)]
#[web::put("/{id}")]
async fn update(
  db: web::types::State<SqlitePool>,
  path: web::types::Path<u32>,
  user: web::types::Json<UserUpdate>,
) -> Result<Response, HttpError> {
  response_json!(service::update(db.get_ref(), path.into_inner(), user.into_inner()))?;
  Ok(web::HttpResponse::Ok().finish())
}

#[utoipa::path(
  delete,
  path = "/{id}",
  params(("id" = u32, Path),),
  responses(
    (status = 200, description = "User deleted"),
    (status = 404, description = "User not found", body = HttpError),
  ),
)]
#[web::delete("/{id}")]
async fn delete(db: web::types::State<SqlitePool>, path: web::types::Path<i64>) -> Result<Response, HttpError> {
  response_json!(service::delete(db.get_ref(), path.into_inner()))?;
  Ok(web::HttpResponse::Ok().finish())
}
