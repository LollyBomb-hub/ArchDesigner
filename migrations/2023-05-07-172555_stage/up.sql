-- Your SQL goes here

CREATE SEQUENCE stages_id_seq;

CREATE TABLE stages
(
    stage_id INTEGER NOT NULL PRIMARY KEY DEFAULT nextval('stages_id_seq'),
    project_id INTEGER NOT NULL REFERENCES projects(project_id),
    mesh_id INTEGER NOT NULL REFERENCES meshs(mesh_id),
    account_id INTEGER NOT NULL REFERENCES accounts(account_id),
    stage_description VARCHAR,
    position_z FLOAT NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc'),
    stars INTEGER NOT NULL DEFAULT 0
);

ALTER SEQUENCE stages_id_seq
    OWNED BY stages.stage_id;