-- Add up migration script here
CREATE TABLE comments (
    id VARCHAR PRIMARY KEY,
    by VARCHAR NOT NULL,
    parent_item_id VARCHAR NOT NULL,
    parent_item_title VARCHAR NOT NULL,
    is_parent BOOLEAN NOT NULL,
    parent_comment_id VARCHAR,
    path TEXT, -- For Materialized Path, store as ltree or text
    text TEXT,
    points INTEGER DEFAULT 1 CHECK (points >= -4),
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    dead BOOLEAN DEFAULT false
);
