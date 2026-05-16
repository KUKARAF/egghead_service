CREATE TABLE html_uploads (
    html_id      TEXT    NOT NULL,
    chunk_index  INTEGER NOT NULL,
    total_chunks INTEGER NOT NULL,
    content      TEXT    NOT NULL,
    created_at   DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (html_id, chunk_index)
);

CREATE INDEX idx_html_uploads_html_id ON html_uploads (html_id);
