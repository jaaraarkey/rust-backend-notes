-- 🐘 PostgreSQL Schema for Smart Notes API
-- Enhanced with PostgreSQL-specific features

-- Enable UUID extension for native UUID support
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create notes table with PostgreSQL optimizations
CREATE TABLE notes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- PostgreSQL-specific constraints
    CONSTRAINT title_not_empty CHECK (length(trim(title)) > 0),
    CONSTRAINT content_not_empty CHECK (length(trim(content)) > 0)
);

-- Create indexes for performance
CREATE INDEX idx_notes_created_at ON notes(created_at DESC);
CREATE INDEX idx_notes_updated_at ON notes(updated_at DESC);
CREATE INDEX idx_notes_title ON notes USING gin(to_tsvector('english', title));
CREATE INDEX idx_notes_content ON notes USING gin(to_tsvector('english', content));

-- Create function to automatically update updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create trigger to auto-update timestamps
CREATE TRIGGER update_notes_updated_at 
    BEFORE UPDATE ON notes 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Insert sample data with PostgreSQL features
INSERT INTO notes (id, title, content, created_at, updated_at) VALUES
(
    uuid_generate_v4(),
    'Welcome to PostgreSQL!', 
    'Your smart auto-title system now runs on enterprise-grade PostgreSQL with advanced indexing, full-text search, and native UUID support.',
    NOW() - INTERVAL '2 hours',
    NOW() - INTERVAL '2 hours'
),
(
    uuid_generate_v4(),
    'PostgreSQL Performance Benefits', 
    'Concurrent connections, advanced indexing, ACID compliance, and horizontal scaling capabilities make PostgreSQL perfect for production applications.',
    NOW() - INTERVAL '1 hour',
    NOW() - INTERVAL '1 hour'
),
(
    uuid_generate_v4(),
    'Full-Text Search Ready!', 
    'PostgreSQL includes built-in full-text search capabilities with GIN indexes for lightning-fast content searches across all your notes.',
    NOW() - INTERVAL '30 minutes',
    NOW() - INTERVAL '30 minutes'
);