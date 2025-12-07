-- Add resume-extracted fields to talents table
ALTER TABLE talents ADD COLUMN resume_experiences TEXT;
ALTER TABLE talents ADD COLUMN linkedin_url TEXT;
ALTER TABLE talents ADD COLUMN x_url TEXT;
ALTER TABLE talents ADD COLUMN github_url TEXT;
ALTER TABLE talents ADD COLUMN gitlab_url TEXT;
