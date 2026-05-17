-- Migration 0008 changed oidc_subject to a partial index, which breaks the
-- ON CONFLICT(oidc_subject) upsert in the OIDC callback. SQLite's upsert
-- conflict targets must reference a table-level UNIQUE/PRIMARY KEY constraint.
--
-- Fix: use a nullable UNIQUE column. SQLite allows multiple NULLs in UNIQUE
-- columns (NULLs don't conflict), so device users (NULL subject) and OIDC
-- users (unique non-null subject) coexist correctly.

CREATE TABLE users_v2 (
    id            TEXT    NOT NULL PRIMARY KEY,
    oidc_subject  TEXT    UNIQUE,
    email         TEXT    NOT NULL,
    created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    balance_cents INTEGER NOT NULL DEFAULT 0
);

INSERT INTO users_v2 (id, oidc_subject, email, created_at, balance_cents)
SELECT id, oidc_subject, email, created_at, balance_cents FROM users;

DROP TABLE users;
ALTER TABLE users_v2 RENAME TO users;

-- Drop the partial index from 0008 (replaced by the column-level UNIQUE above).
DROP INDEX IF EXISTS idx_users_oidc_subject;

-- Keep one device account per email address.
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_email_device
    ON users(email)
    WHERE oidc_subject IS NULL;
