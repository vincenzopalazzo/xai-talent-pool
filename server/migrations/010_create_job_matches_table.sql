-- Job matches table to store candidate rankings for each job
CREATE TABLE IF NOT EXISTS job_matches (
    id TEXT PRIMARY KEY,
    job_id TEXT NOT NULL,
    talent_id TEXT NOT NULL,
    score REAL NOT NULL,
    rank INTEGER NOT NULL,
    match_reasons TEXT,  -- JSON array
    concerns TEXT,       -- JSON array
    summary TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE,
    FOREIGN KEY (talent_id) REFERENCES talents(id) ON DELETE CASCADE,
    UNIQUE(job_id, talent_id)
);

-- Index for fast lookup by job
CREATE INDEX IF NOT EXISTS idx_job_matches_job_id ON job_matches(job_id);

-- Index for ordering by rank
CREATE INDEX IF NOT EXISTS idx_job_matches_rank ON job_matches(job_id, rank);
