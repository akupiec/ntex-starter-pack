use derive_more::{Display, Error};
use ntex::{http, web};
use utoipa::ToSchema;

#[derive(Debug, Display, Error, ToSchema)]
pub enum HttpError {
  #[display("Validation error on field: {}", field)]
  ValidationError {
    field: &'static str,
  },
  InternalError {
    msg: String,
  },
}

impl web::error::WebResponseError for HttpError {
  fn status_code(&self) -> http::StatusCode {
    match *self {
      HttpError::ValidationError { .. } => http::StatusCode::BAD_REQUEST,
      HttpError::InternalError { .. } => http::StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
    web::HttpResponse::build(self.status_code())
      .set_header("content-type", "text/html; charset=utf-8")
      .body(self.to_string())
  }
}
