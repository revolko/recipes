CREATE TABLE recipes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    instructions TEXT NOT NULL,
    cuisine VARCHAR NOT NULL,
    duration_min INTEGER NOT NULL,
    preparation_needed BOOLEAN NOT NULL,
    portions INTEGER NOT NULL,
    difficulty INTEGER NOT NULL
);

CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE ingredients (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE recipe_category (
    recipe_id SERIAL REFERENCES recipes(id),
    category_id SERIAL REFERENCES categories(id),
    PRIMARY KEY (recipe_id, category_id)
);

CREATE TABLE recipe_ingredient (
    recipe_id SERIAL REFERENCES recipes(id),
    ingredient_id SERIAL REFERENCES ingredients(id),
    part SMALLINT NOT NULL,
    quantity NUMERIC(3) NOT NULL,
    unit VARCHAR NOT NULL,
    PRIMARY KEY (recipe_id, ingredient_id)
);
