CREATE TABLE IF NOT EXISTS hiring_requirements (
    id TEXT PRIMARY KEY NOT NULL,
    job_id TEXT,
    title TEXT NOT NULL,
    company_name TEXT NOT NULL,
    requirements_text TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE SET NULL
)
