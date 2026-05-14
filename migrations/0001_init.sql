CREATE TABLE IF NOT EXISTS users (
    id            TEXT NOT NULL PRIMARY KEY,
    oidc_subject  TEXT NOT NULL UNIQUE,
    email         TEXT NOT NULL,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    balance_cents INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS session_tokens (
    id           TEXT NOT NULL PRIMARY KEY,
    token_hash   TEXT NOT NULL UNIQUE,
    oidc_subject TEXT NOT NULL,
    email        TEXT NOT NULL,
    expires_at   TEXT NOT NULL,
    created_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS api_tokens (
    id           TEXT NOT NULL PRIMARY KEY,
    token_hash   TEXT NOT NULL UNIQUE,
    user_id      TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    label        TEXT NOT NULL DEFAULT 'malpa',
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    last_used_at TEXT,
    revoked_at   TEXT
);

CREATE TABLE IF NOT EXISTS tasks (
    id                    TEXT NOT NULL PRIMARY KEY,
    user_id               TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tab_url               TEXT NOT NULL,
    prompt                TEXT NOT NULL,
    page_html             TEXT NOT NULL,
    action_recording      TEXT,
    status                TEXT NOT NULL DEFAULT 'pending'
                              CHECK(status IN (
                                  'pending', 'estimating', 'awaiting_approval',
                                  'processing', 'done', 'failed', 'rejected'
                              )),
    estimated_price_cents INTEGER,
    price_rationale       TEXT,
    approved_at           TEXT,
    rejected_at           TEXT,
    script_name           TEXT,
    script_code           TEXT,
    match_pattern         TEXT,
    error_message         TEXT,
    worker_started_at     TEXT,
    created_at            TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at            TEXT NOT NULL DEFAULT (datetime('now'))
);
