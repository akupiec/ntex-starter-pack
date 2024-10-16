use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, ToSchema)]
pub struct Todo {
  pub id: i32,
  pub title: String,
  pub completed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TodoPartial {
  pub title: String,
}
