-- Add migration script here

DROP TABLE IF EXISTS comments;

CREATE TABLE comments (
    id UUID PRIMARY KEY,
    by TEXT NOT NULL,
    parent_item_id UUID NOT NULL,
    parent_item_title TEXT NOT NULL,
    text TEXT NOT NULL,
    is_parent BOOLEAN NOT NULL,
    root_comment_id UUID NOT NULL,
    parent_comment_id UUID,
    children_count INT DEFAULT 0 Not NULL,
    points INT DEFAULT 1 CHECK (points >= -4) NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    dead BOOLEAN DEFAULT false NOT NULL,
    -- references the id column of items table; when an item is deleted, cascade to children.
    -- FOREIGN KEY (parent_item_id) REFERENCES items(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_comment_id) REFERENCES comments(id) ON DELETE CASCADE
);