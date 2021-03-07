-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body VARCHAR NOT NULL,
    author INT NOT NULL,
    tags VARCHAR NOT NULL,
    permission INT NOT NULL
);