-- Your SQL goes here

CREATE SEQUENCE meshs_id_seq;

CREATE TABLE meshs
(
    mesh_id INTEGER NOT NULL PRIMARY KEY DEFAULT nextval('meshs_id_seq'),
    account_id INTEGER NOT NULL REFERENCES accounts(account_id),
    mesh_name VARCHAR NOT NULL UNIQUE,
    mesh_description VARCHAR NOT NULL,
    ply_contents VARCHAR NOT NULL,
    uploaded_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc'),
    stars INTEGER NOT NULL DEFAULT 0
);

ALTER SEQUENCE meshs_id_seq
    OWNED BY meshs.mesh_id;