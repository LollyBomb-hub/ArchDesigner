-- This file should undo anything in `up.sql`

DROP SEQUENCE IF EXISTS stages_id_seq;

DROP TABLE IF EXISTS stages CASCADE;