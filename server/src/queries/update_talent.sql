UPDATE talents
SET
    name = ?,
    email = ?,
    handle = ?,
    avatar = ?,
    title = ?,
    location = ?,
    experience = ?,
    skills = ?,
    bio = ?,
    verified = ?
WHERE id = ?
RETURNING *;