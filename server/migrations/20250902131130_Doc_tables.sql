
CREATE TABLE documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id INT REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    content TEXT DEFAULT '',
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now()
);

CREATE TABLE document_collaborators (
    doc_id UUID REFERENCES documents(id) ON DELETE CASCADE,
    user_id INT REFERENCES users(id) ON DELETE CASCADE,
    role TEXT CHECK (role IN ('reader', 'editor')) NOT NULL,
    PRIMARY KEY (doc_id, user_id)
);

ALTER TABLE documents
ALTER COLUMN owner_id SET NOT NULL;
