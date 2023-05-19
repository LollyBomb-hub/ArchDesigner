-- This file should undo anything in `up.sql`

DROP SEQUENCE IF EXISTS models_id_seq;

DROP TABLE IF EXISTS models CASCADE;