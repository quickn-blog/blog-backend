-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    pass VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    nickname VARCHAR NOT NULL,
    permission INT NOT NULL
);