CREATE TABLE IF NOT EXISTS talents (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    handle TEXT NOT NULL,
    avatar TEXT,
    title TEXT NOT NULL,
    location TEXT,
    experience TEXT NOT NULL,
    skills TEXT NOT NULL,  -- comma-separated string for skills
    bio TEXT,
    verified INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);