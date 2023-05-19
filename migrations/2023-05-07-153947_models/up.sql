-- Your SQL goes here

CREATE SEQUENCE models_id_seq;

CREATE TABLE models
(
    model_id INTEGER NOT NULL PRIMARY KEY DEFAULT nextval('models_id_seq'),
    account_id INTEGER NOT NULL REFERENCES accounts(account_id),
    model_name VARCHAR NOT NULL UNIQUE,
    model_description VARCHAR NOT NULL,
    model_ifc VARCHAR NOT NULL,
    uploaded_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc'),
    stars INTEGER NOT NULL DEFAULT 0
);

ALTER SEQUENCE models_id_seq
    OWNED BY models.model_id;