INSERT INTO talents (id, name, email, handle, avatar, title, location, experience, skills, bio, verified, created_at)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *; -- to get inserted row