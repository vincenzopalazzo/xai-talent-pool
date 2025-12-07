INSERT INTO jobs (
    id, title, description, company_name, company_logo, location,
    location_type, employment_type, salary_min, salary_max, salary_currency,
    skills_required, experience_level, status, created_at, expires_at
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *
