CREATE TABLE IF NOT EXISTS analytics (
    timestamp INTEGER NOT NULL,
    ip INTEGER NOT NULL,
    path TEXT NOT NULL,
    referer TEXT,
    user_agent TEXT
);