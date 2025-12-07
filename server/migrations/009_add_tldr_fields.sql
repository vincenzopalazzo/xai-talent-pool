-- Add TLDR summary fields for each social platform
ALTER TABLE talents ADD COLUMN github_tldr TEXT;
ALTER TABLE talents ADD COLUMN linkedin_tldr TEXT;
ALTER TABLE talents ADD COLUMN twitter_tldr TEXT;
ALTER TABLE talents ADD COLUMN stackoverflow_tldr TEXT;
