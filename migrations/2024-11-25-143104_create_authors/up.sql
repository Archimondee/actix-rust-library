-- Your SQL goes here
CREATE TABLE authors (
  id TEXT PRIMARY KEY NOT NULL,
  firstname TEXT NOT NULL,
  lastname TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);