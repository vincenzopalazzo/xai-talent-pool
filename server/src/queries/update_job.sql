UPDATE jobs SET
    title = CASE WHEN ?1 != '' THEN ?1 ELSE title END,
    description = CASE WHEN ?2 != '' THEN ?2 ELSE description END,
    company_name = CASE WHEN ?3 != '' THEN ?3 ELSE company_name END,
    company_logo = COALESCE(?4, company_logo),
    location = COALESCE(?5, location),
    location_type = CASE WHEN ?6 != '' THEN ?6 ELSE location_type END,
    employment_type = CASE WHEN ?7 != '' THEN ?7 ELSE employment_type END,
    salary_min = COALESCE(?8, salary_min),
    salary_max = COALESCE(?9, salary_max),
    salary_currency = COALESCE(?10, salary_currency),
    skills_required = CASE WHEN ?11 != '' THEN ?11 ELSE skills_required END,
    experience_level = CASE WHEN ?12 != '' THEN ?12 ELSE experience_level END,
    status = CASE WHEN ?13 != '' THEN ?13 ELSE status END,
    expires_at = COALESCE(?14, expires_at)
WHERE id = ?15
RETURNING *
