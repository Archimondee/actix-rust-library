-- Your SQL goes here
CREATE TABLE book_transaction_logs (
  id TEXT PRIMARY KEY NOT NULL,
  book_transaction_id TEXT NOT NULL,
  action TEXT NOT NULL, -- create, update, delete
  action_status TEXT NOT NULL, -- borrow, return, purchase
  action_timestamp TIMESTAMP NOT NULL,
  performed_by TEXT NOT NULL,
  FOREIGN KEY (book_transaction_id) REFERENCES book_transactions_id
);