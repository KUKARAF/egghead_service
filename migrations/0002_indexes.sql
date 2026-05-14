CREATE INDEX IF NOT EXISTS idx_tasks_user_id        ON tasks(user_id);
CREATE INDEX IF NOT EXISTS idx_tasks_status          ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_api_tokens_hash       ON api_tokens(token_hash);
CREATE INDEX IF NOT EXISTS idx_session_tokens_hash   ON session_tokens(token_hash);
