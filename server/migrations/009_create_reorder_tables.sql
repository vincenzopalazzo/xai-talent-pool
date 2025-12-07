-- Migration 007: Create reorder tracking tables for manual candidate ranking

-- Reorder Events Table
-- Stores raw reorder events with before/after snapshots
CREATE TABLE IF NOT EXISTS reorder_events (
    id TEXT PRIMARY KEY,
    job_id TEXT NOT NULL,
    before_order TEXT NOT NULL,  -- JSON array of talent IDs in order before reorder
    after_order TEXT NOT NULL,   -- JSON array of talent IDs in order after reorder
    moved_talent_id TEXT,        -- Optional: specific talent that was moved
    event_timestamp TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_reorder_events_job_id ON reorder_events(job_id);
CREATE INDEX IF NOT EXISTS idx_reorder_events_timestamp ON reorder_events(event_timestamp);

-- Pairwise Preferences Table
-- Stores derived preferences (winner ≻ loser) from reorder events
CREATE TABLE IF NOT EXISTS pairwise_preferences (
    id TEXT PRIMARY KEY,
    winner_id TEXT NOT NULL,     -- Talent ID of preferred candidate
    loser_id TEXT NOT NULL,      -- Talent ID of less preferred candidate
    job_id TEXT NOT NULL,
    job_text TEXT NOT NULL,      -- Human-readable job title/description
    winner_text TEXT NOT NULL,   -- Human-readable candidate info (moved up)
    loser_text TEXT NOT NULL,    -- Human-readable candidate info (moved below)
    source TEXT NOT NULL DEFAULT 'manual_reorder',
    confidence REAL NOT NULL DEFAULT 1.0,
    reorder_event_id TEXT,       -- Reference to originating reorder event
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE,
    FOREIGN KEY (reorder_event_id) REFERENCES reorder_events(id) ON DELETE CASCADE,
    -- Ensure uniqueness: same preference can't be derived multiple times from same event
    UNIQUE(winner_id, loser_id, job_id, reorder_event_id)
);

CREATE INDEX IF NOT EXISTS idx_pairwise_preferences_job_id ON pairwise_preferences(job_id);
CREATE INDEX IF NOT EXISTS idx_pairwise_preferences_winner ON pairwise_preferences(winner_id);
CREATE INDEX IF NOT EXISTS idx_pairwise_preferences_loser ON pairwise_preferences(loser_id);
CREATE INDEX IF NOT EXISTS idx_pairwise_preferences_source ON pairwise_preferences(source);
