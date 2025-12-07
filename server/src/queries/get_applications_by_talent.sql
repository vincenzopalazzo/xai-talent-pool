SELECT id, talent_id, job_id, resume_data, resume_filename, resume_content_type, cover_letter, status, created_at
FROM applications
WHERE talent_id = ?
ORDER BY created_at DESC
