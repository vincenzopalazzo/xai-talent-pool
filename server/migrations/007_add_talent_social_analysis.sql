-- Add social media analysis fields to talents table
ALTER TABLE talents ADD COLUMN social_analysis TEXT;
ALTER TABLE talents ADD COLUMN x_handle_discovered TEXT;