-- Your SQL goes here
CREATE TABLE IF NOT EXISTS bot_users
(
    id         BIGINT PRIMARY KEY,
    username   VARCHAR     NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);