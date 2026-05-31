ALTER TABLE tasks ADD COLUMN script_id TEXT REFERENCES userscripts(id);

CREATE TABLE script_messages (
    id         TEXT NOT NULL PRIMARY KEY,
    script_id  TEXT NOT NULL REFERENCES userscripts(id) ON DELETE CASCADE,
    role       TEXT NOT NULL CHECK(role IN ('user', 'assistant')),
    content    TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_script_messages_script ON script_messages(script_id);
