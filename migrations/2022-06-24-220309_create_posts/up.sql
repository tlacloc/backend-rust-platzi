-- Your SQL goes here1
CREATE TABLE posts (
  id serial PRIMARY KEY,
  title varchar(255) NOT NULL,
  slug varchar(255) NOT NULL,
  body text NOT NULL
);
