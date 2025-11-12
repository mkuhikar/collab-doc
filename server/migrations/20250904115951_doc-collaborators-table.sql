CREATE TYPE role AS ENUM ('reader', 'editor');

CREATE TABLE doc_collaborators (
    doc_id UUID REFERENCES documents(id) ON DELETE CASCADE,
    user_id INT REFERENCES users(id) ON DELETE CASCADE,
    role role NOT NULL,
    PRIMARY KEY (doc_id, user_id)
);