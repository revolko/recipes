-- This file should undo anything in `up.sql`
ALTER TABLE ingredients DROP CONSTRAINT name_unique;
