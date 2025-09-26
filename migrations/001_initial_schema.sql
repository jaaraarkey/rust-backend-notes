-- Create migrations directory first:
-- mkdir -p migrations

-- migrations/001_initial_schema.sql
-- Initial database schema for notes with smart auto-title support

CREATE TABLE IF NOT EXISTS notes (
    -- Primary key with UUID format
    id TEXT PRIMARY KEY NOT NULL,
    
    -- Note content fields
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    
    -- Timestamps in RFC3339 format (ISO 8601)
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    -- Constraints for data integrity
    CHECK (length(id) > 0),
    CHECK (length(title) > 0),
    CHECK (length(content) > 0),
    CHECK (length(created_at) > 0),
    CHECK (length(updated_at) > 0)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_notes_created_at ON notes(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_notes_updated_at ON notes(updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_notes_title ON notes(title);

-- Add sample data for testing the smart auto-title system
INSERT OR IGNORE INTO notes (id, title, content, created_at, updated_at) VALUES 
(
    '550e8400-e29b-41d4-a716-446655440000',
    'Welcome to Smart Notes!',
    'Welcome to Smart Notes! This is your first note with automatic title extraction. The system can intelligently extract titles from your content while preserving everything you write.',
    '2024-01-27T10:00:00Z',
    '2024-01-27T10:00:00Z'
),
(
    '6ba7b810-9dad-11d1-80b4-00c04fd430c8', 
    'How does title extraction work?',
    'How does title extraction work? The system uses multiple strategies: sentence boundaries, exclamation points, question marks, and line breaks to find the perfect title from your content.',
    '2024-01-27T11:30:00Z',
    '2024-01-27T11:30:00Z'
),
(
    '6ba7b811-9dad-11d1-80b4-00c04fd430c8',
    'Database Integration Complete!',
    'Database Integration Complete! Now your notes are stored persistently in SQLite with full CRUD operations, connection pooling, and transaction support for data integrity.',
    '2024-01-27T12:00:00Z',
    '2024-01-27T12:00:00Z'
);