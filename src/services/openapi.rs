use crate::error::HttpError;
use std::sync::Arc;

use ntex::util::Bytes;
use ntex::web;
use utoipa::OpenApi;

use crate::models::todo::{Todo, TodoPartial};

use super::todo;

/// Main structure to generate OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
  paths(
    todo::get_todos,
    todo::create_todo,
    todo::get_todo,
    todo::update_todo,
    todo::delete_todo,
  ),
  components(schemas(Todo, TodoPartial, HttpError))
)]
pub(crate) struct ApiDoc;

#[web::get("/{tail}*")]
async fn get_swagger(
  tail: web::types::Path<String>,
  openapi_conf: web::types::State<Arc<utoipa_swagger_ui::Config<'static>>>,
) -> Result<web::HttpResponse, HttpError> {
  if tail.as_ref() == "swagger.json" {
    let spec = ApiDoc::openapi().to_json().map_err(|err| HttpError::InternalError {
      msg: format!("Error generating OpenAPI spec: {}", err),
    })?;
    return Ok(web::HttpResponse::Ok().content_type("application/json").body(spec));
  }
  let serve_path = utoipa_swagger_ui::serve(&tail, openapi_conf.as_ref().clone().into()).map_err(|err| HttpError::InternalError {
    msg: format!("Error serving Swagger UI: {}", err),
  })?;
  match serve_path {
    None => Ok(web::HttpResponse::NotFound().finish()),
    Some(file) => Ok({
      let bytes = Bytes::from(file.bytes.to_vec());
      web::HttpResponse::Ok().content_type(file.content_type).body(bytes)
    }),
  }
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  let swagger_config = Arc::new(utoipa_swagger_ui::Config::new(["/explorer/swagger.json"]).use_base_layout());
  config.service(web::scope("/explorer/").state(swagger_config).service(get_swagger));
}
