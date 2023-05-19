-- This file should undo anything in `up.sql`
DROP SEQUENCE IF EXISTS projects_id_seq;

DROP TABLE IF EXISTS projects CASCADE;