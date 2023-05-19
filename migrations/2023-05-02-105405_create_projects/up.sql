-- Your SQL goes here

CREATE SEQUENCE projects_id_seq;

CREATE TABLE projects
(
    project_id          INTEGER NOT NULL PRIMARY KEY DEFAULT nextval('projects_id_seq'),
    account_id          INTEGER NOT NULL REFERENCES accounts(account_id),
    project_name        VARCHAR NOT NULL UNIQUE,
    project_description VARCHAR,
    created_at          TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc'),
    stars               INTEGER NOT NULL DEFAULT 0
);

ALTER SEQUENCE projects_id_seq
    OWNED BY projects.account_id;