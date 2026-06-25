CREATE TABLE images (
  id SERIAL PRIMARY KEY,
  bytes BYTEA NOT NULL,
  type VARCHAR NOT NULL,
  recipe_id SERIAL REFERENCES recipes(id)
);
