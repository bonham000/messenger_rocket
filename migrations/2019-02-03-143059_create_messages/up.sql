-- Your SQL goes here
CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    message VARCHAR NOT NULL,
    author VARCHAR NOT NULL
)