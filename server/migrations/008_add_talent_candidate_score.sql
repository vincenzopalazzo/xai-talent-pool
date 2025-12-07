-- Add candidate score fields to talents table
ALTER TABLE talents ADD COLUMN candidate_score REAL;
ALTER TABLE talents ADD COLUMN candidate_score_details TEXT;
