INSERT INTO applications (id, talent_id, job_id, resume_data, resume_filename, resume_content_type, cover_letter, status, created_at)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING id, talent_id, job_id, resume_data, resume_filename, resume_content_type, cover_letter, status, created_at
