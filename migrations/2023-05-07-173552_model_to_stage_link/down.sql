-- This file should undo anything in `up.sql`

DROP SEQUENCE IF EXISTS model_to_stage_link_id_seq;

DROP TABLE IF EXISTS model_to_stage_link CASCADE;