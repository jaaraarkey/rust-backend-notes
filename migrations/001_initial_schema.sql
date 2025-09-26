-- Smart Notes PostgreSQL Schema
-- Production-grade database for smart auto-title system

-- Enable UUID extension for PostgreSQL
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Notes table with advanced PostgreSQL features
CREATE TABLE notes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Add constraints
    CONSTRAINT notes_title_not_empty CHECK (LENGTH(TRIM(title)) > 0),
    CONSTRAINT notes_content_not_empty CHECK (LENGTH(TRIM(content)) > 0)
);

-- Indexes for performance
CREATE INDEX idx_notes_created_at ON notes(created_at DESC);
CREATE INDEX idx_notes_updated_at ON notes(updated_at DESC);
CREATE INDEX idx_notes_title ON notes(title);

-- Full-text search index for content (PostgreSQL magic!)
CREATE INDEX idx_notes_content_search ON notes USING gin(to_tsvector('english', content));
CREATE INDEX idx_notes_title_search ON notes USING gin(to_tsvector('english', title));

-- Function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger to automatically update updated_at on UPDATE
CREATE TRIGGER update_notes_updated_at 
    BEFORE UPDATE ON notes 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Insert some sample data for testing
INSERT INTO notes (title, content) VALUES 
    ('Welcome to PostgreSQL!', 'Your smart auto-title system is now powered by enterprise-grade PostgreSQL with full-text search, advanced indexing, and ACID compliance.'),
    ('Database Upgrade Complete', 'Successfully migrated from SQLite to PostgreSQL. The system now supports concurrent users, advanced queries, and production-scale workloads.'),
    ('PostgreSQL Features', 'Amazing features now available: full-text search, JSON support, advanced indexing, row-level security, and much more!');