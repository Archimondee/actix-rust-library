-- Your SQL goes here
CREATE TABLE books (
  id TEXT PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  author_id TEXT NOT NULL,
  publication_date TIMESTAMP NOT NULL,
  available_copies INT NOT NULL DEFAULT 0,
  category_id TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  FOREIGN KEY (author_id) REFERENCES authors(id),
  FOREIGN KEY (category_id) REFERENCES categories(id)
)