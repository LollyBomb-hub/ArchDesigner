-- This file should undo anything in `up.sql`

DROP SEQUENCE IF EXISTS meshs_id_seq;

DROP TABLE IF EXISTS meshs CASCADE;