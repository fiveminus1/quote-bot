-- Add migration script here
CREATE TABLE IF NOT EXISTS quotes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    quoted_by TEXT NOT NULL,
    quoted_user TEXT NOT NULL,
    quoted_text TEXT NOT NULL,
    quote_time TEXT NOT NULL
);