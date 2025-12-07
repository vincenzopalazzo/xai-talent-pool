CREATE TABLE IF NOT EXISTS applications (
    id TEXT PRIMARY KEY,
    talent_id TEXT NOT NULL,
    job_id TEXT NOT NULL,
    resume_data TEXT,
    resume_filename TEXT,
    resume_content_type TEXT,
    cover_letter TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TEXT NOT NULL,
    FOREIGN KEY (talent_id) REFERENCES talents(id),
    FOREIGN KEY (job_id) REFERENCES jobs(id)
);

CREATE INDEX IF NOT EXISTS idx_applications_talent_id ON applications(talent_id);
CREATE INDEX IF NOT EXISTS idx_applications_job_id ON applications(job_id);
