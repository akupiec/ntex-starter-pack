use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct User {
  pub id: i64,
  pub externalId: Option<String>,
  pub role: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UserUpdate {
  pub externalId: String,
  pub role: String,
}
