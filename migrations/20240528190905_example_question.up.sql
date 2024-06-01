-- Add up migration script here
CREATE TABLE IF NOT EXISTS questions (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    asker TEXT
);

CREATE TABLE IF NOT EXISTS tags (
    id TEXT REFERENCES questions (id),
    tag TEXT NOT NULL
);