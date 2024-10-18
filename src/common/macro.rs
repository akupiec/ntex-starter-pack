#[macro_export]
macro_rules! response_json {
  ($service_call:expr) => {
    match $service_call.await {
      Ok(res) => Ok(web::HttpResponse::Ok().json(&res)),
      Err(e) => Err(e),
    }
  };
}
