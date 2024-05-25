CREATE TABLE IF NOT EXISTS analytics (
    ip INTEGER NOT NULL,
    path TEXT NOT NULL,
    referer TEXT,
    user_agent TEXT
);