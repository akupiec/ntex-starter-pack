CREATE TABLE User
(
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  external_id TEXT UNIQUE,
  role        VARCHAR(50)
);