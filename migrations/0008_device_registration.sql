-- Relax users.oidc_subject to nullable so device-registered users can exist
-- without an OIDC identity. SQLite cannot ALTER NOT NULL, so we recreate the table.
CREATE TABLE users_new (
    id            TEXT    NOT NULL PRIMARY KEY,
    oidc_subject  TEXT,
    email         TEXT    NOT NULL,
    created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    balance_cents INTEGER NOT NULL DEFAULT 0
);

INSERT INTO users_new (id, oidc_subject, email, created_at, balance_cents)
SELECT id, oidc_subject, email, created_at, balance_cents FROM users;

DROP TABLE users;
ALTER TABLE users_new RENAME TO users;

-- OIDC users remain unique by subject (NULLs excluded from uniqueness).
CREATE UNIQUE INDEX idx_users_oidc_subject
    ON users(oidc_subject)
    WHERE oidc_subject IS NOT NULL;

-- Device users: one account per email (only enforced when oidc_subject IS NULL).
CREATE UNIQUE INDEX idx_users_email_device
    ON users(email)
    WHERE oidc_subject IS NULL;

-- Stores long-lived tokens issued to approved Electron app instances.
CREATE TABLE device_tokens (
    id           TEXT NOT NULL PRIMARY KEY,
    token_hash   TEXT NOT NULL UNIQUE,
    user_id      TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_name  TEXT NOT NULL,
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at   TEXT NOT NULL,
    revoked_at   TEXT,
    last_used_at TEXT
);

CREATE INDEX idx_device_tokens_user ON device_tokens(user_id, revoked_at);

-- Tracks pending registration attempts from Electron app instances.
CREATE TABLE device_registrations (
    id           TEXT NOT NULL PRIMARY KEY,
    name         TEXT NOT NULL,
    email        TEXT NOT NULL,
    emoji_hash   TEXT NOT NULL,
    status       TEXT NOT NULL DEFAULT 'pending'
                     CHECK(status IN ('pending', 'approved', 'rejected', 'expired')),
    token_plain  TEXT,
    user_id      TEXT REFERENCES users(id),
    device_token_id TEXT REFERENCES device_tokens(id),
    requested_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at   TEXT NOT NULL,
    approved_at  TEXT,
    rejected_at  TEXT
);

CREATE INDEX idx_device_reg_status ON device_registrations(status, expires_at);
