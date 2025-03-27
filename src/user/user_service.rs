use crate::infra::model::HttpError;
use crate::user::model::{User, UserUpdate};
use sqlx::{query, query_as, Error, SqlitePool};

fn map_db_error(e: Error) -> HttpError {
  HttpError::InternalError { msg: e.to_string() }
}

macro_rules! empty_db_resp {
  ($db:expr, $query:expr, $($args:tt)*) => {
    match query!($query, $($args)*).execute($db).await {
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
  db_resp!(query_as!(User, "SELECT * FROM User").fetch_all(db))
}

pub async fn delete(db: &SqlitePool, id: i64) -> Result<(), HttpError> {
  empty_db_resp!(db, "DELETE FROM User WHERE id = ?", id)
}

pub async fn save(db: &SqlitePool, user: UserUpdate) -> Result<(), HttpError> {
  empty_db_resp!(
    db,
    "INSERT INTO User (external_id, role) VALUES ($1, $2)",
    user.external_id,
    user.role
  )
}

pub async fn find(db: &SqlitePool, id: u32) -> Result<User, HttpError> {
  db_resp!(query_as!(User, "SELECT * FROM User WHERE id = ?", id).fetch_one(db))
}

pub async fn update(db: &SqlitePool, id: u32, user: UserUpdate) -> Result<(), HttpError> {
  let not_found = db_resp!(query!("SELECT id FROM User WHERE id = ?", id).fetch_optional(db))?.is_none();

  if not_found {
    return Err(HttpError::ValidationError { msg: "not found User!" });
  }

  empty_db_resp!(
    db,
    "UPDATE User SET role = ?, external_id = ? WHERE id = ?",
    user.role,
    user.external_id,
    id
  )
}
