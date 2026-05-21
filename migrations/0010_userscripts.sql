-- Store userscripts with metadata and Violentmonkey format
CREATE TABLE userscripts (
    id           TEXT NOT NULL PRIMARY KEY,
    user_id      TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name         TEXT NOT NULL,
    prompt       TEXT NOT NULL,
    url          TEXT NOT NULL,
    script_code  TEXT NOT NULL,
    violentmonkey_metadata TEXT,
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_userscripts_user ON userscripts(user_id);

-- Many-to-many relationship: scripts can be assigned to multiple devices
CREATE TABLE script_device_assignments (
    id          TEXT NOT NULL PRIMARY KEY,
    script_id   TEXT NOT NULL REFERENCES userscripts(id) ON DELETE CASCADE,
    device_token_id TEXT NOT NULL REFERENCES device_tokens(id) ON DELETE CASCADE,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(script_id, device_token_id)
);

CREATE INDEX idx_script_device_assignments_script ON script_device_assignments(script_id);
CREATE INDEX idx_script_device_assignments_device ON script_device_assignments(device_token_id);
