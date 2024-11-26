-- Your SQL goes here
CREATE TABLE book_favorites (
  id TEXT PRIMARY KEY NOT NULL,
  book_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  FOREIGN KEY (book_id) REFERENCES books(id),
  FOREIGN KEY (user_id) REFERENCES users(id)
)