-- Add migration script here
-- Ensure content is NOT NULL with a default
ALTER TABLE documents
    ALTER COLUMN content SET DEFAULT '',
    ALTER COLUMN content SET NOT NULL;

-- Ensure created_at is NOT NULL with a default
ALTER TABLE documents
    ALTER COLUMN created_at SET DEFAULT now(),
    ALTER COLUMN created_at SET NOT NULL;

-- Ensure updated_at is NOT NULL with a default
ALTER TABLE documents
    ALTER COLUMN updated_at SET DEFAULT now(),
    ALTER COLUMN updated_at SET NOT NULL;