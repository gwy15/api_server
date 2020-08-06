-- Your SQL goes here
CREATE TABLE "users" (
    id SERIAL PRIMARY KEY,
    username VARCHAR(64) NOT NULL,
    password VARCHAR(512) NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT false,
    is_disabled BOOLEAN NOT NULL DEFAULT false,

    last_login TIMESTAMPTZ NOT NULL,
    token_valid_after TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT diesel_manage_updated_at('users');