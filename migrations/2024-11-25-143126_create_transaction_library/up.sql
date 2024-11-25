-- Your SQL goes here
CREATE TABLE book_transactions ( 
  id TEXT PRIMARY KEY NOT NULL,
  book_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  transaction_type TEXT NOT NULL, -- borrow, return, purchase
  transaction_date TIMESTAMP NOT NULL,
  due_date TIMESTAMP NOT NULL,
  return_date TIMESTAMP,
  quantity INT NOT NULL DEFAULT 1,
  created_at TIMESTAMP NOT NULL,
  FOREIGN KEY (book_id) REFERENCES books(id),
  FOREIGN KEY (user_id) REFERENCES users(id)
);