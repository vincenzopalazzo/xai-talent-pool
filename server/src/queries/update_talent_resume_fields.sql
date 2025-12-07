UPDATE talents
SET resume_experiences = ?,
    linkedin_url = ?,
    x_url = ?,
    github_url = ?,
    gitlab_url = ?
WHERE id = ?
RETURNING *;
