-- Your SQL goes here

CREATE SEQUENCE accounts_id_seq;

CREATE TABLE accounts
(
    account_id INTEGER NOT NULL PRIMARY KEY DEFAULT nextval('accounts_id_seq'),
    "name" VARCHAR NOT NULL UNIQUE,
    "password" VARCHAR NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc'),
    stars INTEGER NOT NULL DEFAULT (0),
    about VARCHAR
);

ALTER SEQUENCE accounts_id_seq
    OWNED BY accounts.account_id;