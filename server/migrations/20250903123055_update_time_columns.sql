-- Add migration script here
ALTER TABLE documents
    ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at::timestamptz,
    ALTER COLUMN created_at SET DEFAULT now(),
    ALTER COLUMN updated_at TYPE TIMESTAMPTZ USING updated_at::timestamptz,
    ALTER COLUMN updated_at SET DEFAULT now();
