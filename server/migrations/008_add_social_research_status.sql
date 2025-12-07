-- Add social research status field to talents table
-- Values: pending, in_progress, completed, failed
ALTER TABLE talents ADD COLUMN social_research_status TEXT;
