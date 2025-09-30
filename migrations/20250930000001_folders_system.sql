-- Create folders table with hierarchical support
CREATE TABLE IF NOT EXISTS folders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL CHECK (length(name) > 0 AND length(name) <= 100),
    description TEXT,
    color TEXT DEFAULT '#3B82F6',
    icon TEXT DEFAULT 'folder',
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES folders(id) ON DELETE CASCADE,
    position INTEGER DEFAULT 0,
    is_default BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Prevent circular references
    CONSTRAINT folders_no_self_parent CHECK (id != parent_id),
    -- Unique folder names per user per parent
    CONSTRAINT folders_unique_name_per_parent UNIQUE (user_id, parent_id, name)
);

-- Add folder support to notes table (only if columns don't exist)
DO $$ 
BEGIN 
    -- Check and add folder_id column
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'notes' AND column_name = 'folder_id'
    ) THEN
        ALTER TABLE notes ADD COLUMN folder_id UUID REFERENCES folders(id) ON DELETE SET NULL;
    END IF;

    -- Check and add is_pinned column
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'notes' AND column_name = 'is_pinned'
    ) THEN
        ALTER TABLE notes ADD COLUMN is_pinned BOOLEAN DEFAULT FALSE;
    END IF;

    -- Check and add pinned_at column
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'notes' AND column_name = 'pinned_at'
    ) THEN
        ALTER TABLE notes ADD COLUMN pinned_at TIMESTAMPTZ;
    END IF;

    -- Check and add view_count column
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'notes' AND column_name = 'view_count'
    ) THEN
        ALTER TABLE notes ADD COLUMN view_count INTEGER DEFAULT 0;
    END IF;

    -- Check and add word_count column
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'notes' AND column_name = 'word_count'
    ) THEN
        ALTER TABLE notes ADD COLUMN word_count INTEGER DEFAULT 0;
    END IF;
END $$;

-- Create indexes for performance (only if they don't exist)
CREATE INDEX IF NOT EXISTS folders_user_id_idx ON folders(user_id);
CREATE INDEX IF NOT EXISTS folders_parent_id_idx ON folders(parent_id);
CREATE INDEX IF NOT EXISTS folders_position_idx ON folders(user_id, parent_id, position);
CREATE INDEX IF NOT EXISTS notes_folder_id_idx ON notes(folder_id);
CREATE INDEX IF NOT EXISTS notes_pinned_idx ON notes(user_id, is_pinned, pinned_at);

-- Create a function to update word count automatically
CREATE OR REPLACE FUNCTION update_note_word_count()
RETURNS TRIGGER AS $$
BEGIN
    NEW.word_count = array_length(string_to_array(trim(NEW.content), ' '), 1);
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Drop existing trigger if it exists, then create new one
DROP TRIGGER IF EXISTS trigger_update_note_word_count ON notes;
CREATE TRIGGER trigger_update_note_word_count
    BEFORE INSERT OR UPDATE ON notes
    FOR EACH ROW
    EXECUTE FUNCTION update_note_word_count();

-- Create default folders for existing users (only if they don't have one)
INSERT INTO folders (name, description, color, icon, user_id, is_default)
SELECT 
    'My Notes' as name,
    'Default folder for all your notes' as description,
    '#3B82F6' as color,
    'folder' as icon,
    u.id as user_id,
    TRUE as is_default
FROM users u
WHERE NOT EXISTS (
    SELECT 1 FROM folders f 
    WHERE f.user_id = u.id AND f.is_default = TRUE
);

-- Update existing notes to be in default folders (only unassigned notes)
UPDATE notes 
SET folder_id = (
    SELECT f.id 
    FROM folders f 
    WHERE f.user_id = notes.user_id 
    AND f.is_default = TRUE 
    LIMIT 1
)
WHERE folder_id IS NULL 
AND user_id IS NOT NULL;