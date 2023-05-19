-- Your SQL goes here

CREATE SEQUENCE model_to_stage_link_id_seq;

CREATE TABLE model_to_stage_link
(
    link_id INTEGER NOT NULL PRIMARY KEY DEFAULT nextval('model_to_stage_link_id_seq'),
    model_id INTEGER NOT NULL REFERENCES models(model_id),
    stage_id INTEGER NOT NULL REFERENCES stages(stage_id),
    account_id INTEGER NOT NULL REFERENCES accounts(account_id),
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc'),
    link_description VARCHAR,
    position_x FLOAT NOT NULL,
    position_y FLOAT NOT NULL,
    -- Nullable since posizion_z can be taken from stage!
    position_z FLOAT,
    rotation_x FLOAT NOT NULL,
    rotation_y FLOAT NOT NULL,
    rotation_z FLOAT NOT NULL
);

ALTER SEQUENCE model_to_stage_link_id_seq
    OWNED BY model_to_stage_link.link_id;