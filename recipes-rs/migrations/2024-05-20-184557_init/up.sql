CREATE TABLE recipes (
    recipe_id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    instructions TEXT NOT NULL,
    cuisine VARCHAR NOT NULL,
    duration_min INTEGER NOT NULL,
    preparation_needed BOOLEAN NOT NULL,
    portions INTEGER NOT NULL,
    difficulty INTEGER NOT NULL
)
