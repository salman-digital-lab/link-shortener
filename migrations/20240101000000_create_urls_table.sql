CREATE TABLE IF NOT EXISTS urls (
    id VARCHAR(10) PRIMARY KEY,
    original_url TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    visit_count BIGINT NOT NULL DEFAULT 0
);

CREATE INDEX idx_original_url ON urls(original_url);
