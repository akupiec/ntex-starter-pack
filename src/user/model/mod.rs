use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct User {
  pub id: i64,
  pub external_id: Option<String>,
  pub role: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UserUpdate {
  pub external_id: String,
  pub role: String,
}
