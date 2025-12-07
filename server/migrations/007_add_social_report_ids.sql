-- Add social media research report document IDs to talents table
ALTER TABLE talents ADD COLUMN github_report_id TEXT;
ALTER TABLE talents ADD COLUMN linkedin_report_id TEXT;
ALTER TABLE talents ADD COLUMN twitter_report_id TEXT;
ALTER TABLE talents ADD COLUMN stackoverflow_report_id TEXT;
