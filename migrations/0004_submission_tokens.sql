-- Submission tokens for read-only task access
ALTER TABLE tasks ADD COLUMN submission_token TEXT UNIQUE;

CREATE TABLE submission_tokens (
    token TEXT NOT NULL PRIMARY KEY,
    task_id TEXT NOT NULL UNIQUE REFERENCES tasks(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_submission_tokens_task ON submission_tokens(task_id);
