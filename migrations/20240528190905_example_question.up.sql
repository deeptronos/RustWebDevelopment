-- Add up migration script here
CREATE TABLE IF NOT EXISTS example_questions (
    id INT PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    asker TEXT
)

CREATE TABLE IF NOT EXISTS tags (
    id INT REFERENCES example_questions (id),
    tag TEXT NOT NULL
)