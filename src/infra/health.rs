use ntex::web;

#[utoipa::path(
    get,
    path = "/health",
    description = "Healthy response",
    responses(
    (status = 200)
    ))]
#[web::get("/health")]
pub async fn health_check() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}
