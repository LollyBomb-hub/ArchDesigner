-- This file should undo anything in `up.sql`
DROP SEQUENCE IF EXISTS accounts_id_seq;

DROP TABLE IF EXISTS accounts CASCADE;