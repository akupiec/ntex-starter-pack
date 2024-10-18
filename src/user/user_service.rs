use crate::infra::model::HttpError;
use crate::user::model::{User, UserUpdate};
use sqlx::{query, query_as, Error, SqlitePool};

fn map_db_error(e: Error) -> HttpError {
  HttpError::InternalError { msg: e.to_string() }
}

macro_rules! empty_db_resp {
  ($expr:expr) => {
    match $expr.await {
      Ok(_) => Ok(()),
      Err(e) => Err(map_db_error(e)),
    }
  };
}

macro_rules! db_resp {
  ($expr:expr) => {
    match $expr.await {
      Ok(r) => Ok(r),
      Err(e) => Err(map_db_error(e)),
    }
  };
}

pub async fn find_all(db: &SqlitePool) -> Result<Vec<User>, HttpError> {
  db_resp!(query_as!(User, "SELECT * FROM user").fetch_all(db))
}

pub async fn delete(db: &SqlitePool, id: i64) -> Result<(), HttpError> {
  empty_db_resp!(query!("DELETE FROM user WHERE id = ?", id).execute(db))
}

pub async fn save(db: &SqlitePool, user: UserUpdate) -> Result<(), HttpError> {
  empty_db_resp!(query!(
    "INSERT INTO user (externalId, role) VALUES ($1, $2)",
    user.externalId,
    user.role
  )
  .execute(db))
}

pub async fn find(db: &SqlitePool, id: u32) -> Result<User, HttpError> {
  db_resp!(query_as!(User, "SELECT * FROM user WHERE id = ?", id).fetch_one(db))
}

pub async fn update(db: &SqlitePool, id: u32, user: UserUpdate) -> Result<(), HttpError> {
  let not_found = db_resp!(query!("SELECT id FROM user WHERE id = ?", id).fetch_optional(db))?.is_none();

  if not_found {
    return Err(HttpError::ValidationError { msg: "not found user!" });
  }

  empty_db_resp!(query!(
    "UPDATE user SET role = ?, externalId = ? WHERE id = ?",
    user.role,
    user.externalId,
    id
  )
  .execute(db))
}
