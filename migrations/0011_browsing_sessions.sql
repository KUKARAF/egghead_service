CREATE TABLE browsing_sessions (
    id         TEXT NOT NULL PRIMARY KEY,
    user_id    TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name       TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_browsing_sessions_user ON browsing_sessions(user_id);

CREATE TABLE session_pages (
    id         TEXT NOT NULL PRIMARY KEY,
    session_id TEXT NOT NULL REFERENCES browsing_sessions(id) ON DELETE CASCADE,
    url        TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_session_pages_session ON session_pages(session_id);

CREATE TABLE session_errors (
    id          TEXT NOT NULL PRIMARY KEY,
    session_id  TEXT NOT NULL REFERENCES browsing_sessions(id) ON DELETE CASCADE,
    url         TEXT NOT NULL,
    page_html   TEXT NOT NULL,
    reported_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_session_errors_session ON session_errors(session_id);
