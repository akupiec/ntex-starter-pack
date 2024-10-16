use super::model::HttpError;
use super::ApiDoc;
use ntex::service::Identity;
use ntex::util::Bytes;
use ntex::web;
use ntex::web::{DefaultError, Scope};
use std::sync::Arc;
use utoipa::OpenApi;

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
  let serve_path =
    utoipa_swagger_ui::serve(&tail, openapi_conf.as_ref().clone().into()).map_err(|err| HttpError::InternalError {
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

pub fn ntex_service() -> Scope<DefaultError, Identity> {
  let swagger_config = Arc::new(utoipa_swagger_ui::Config::new(["/explorer/swagger.json"]).use_base_layout());
  web::scope("/explorer/").state(swagger_config).service(get_swagger)
}
