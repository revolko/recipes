-- Your SQL goes here
ALTER TABLE ingredients ADD CONSTRAINT name_unique UNIQUE (name);
