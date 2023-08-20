-- Your SQL goes here
CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  salt TEXT NOT NULL
);

CREATE TABLE cookies(
  id TEXT NOT NULL PRIMARY KEY,
  user_id int NOT NULL,
  expires DATE NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id)
)