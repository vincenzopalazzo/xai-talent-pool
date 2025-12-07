CREATE TABLE IF NOT EXISTS jobs (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    company_name TEXT NOT NULL,
    company_logo TEXT,
    location TEXT,
    location_type TEXT NOT NULL DEFAULT 'remote',
    employment_type TEXT NOT NULL DEFAULT 'full-time',
    salary_min INTEGER,
    salary_max INTEGER,
    salary_currency TEXT,
    skills_required TEXT NOT NULL,
    experience_level TEXT NOT NULL DEFAULT 'mid',
    status TEXT NOT NULL DEFAULT 'active',
    created_at TEXT NOT NULL,
    expires_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_jobs_status ON jobs(status);
CREATE INDEX IF NOT EXISTS idx_jobs_created_at ON jobs(created_at);
