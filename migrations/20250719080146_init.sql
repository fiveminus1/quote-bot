-- Add migration script here
CREATE TABLE IF NOT EXISTS quotes (
    id SERIAL PRIMARY KEY,
    quoted_by TEXT NOT NULL,
    quoted_user TEXT NOT NULL,
    quoted_text TEXT NOT NULL,
    quote_time TIMESTAMPTZ NOT NULL
);