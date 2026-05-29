ALTER TABLE tasks ADD COLUMN device_token_id TEXT REFERENCES device_tokens(id);
