-- Add up migration script here
DROP TABLE IF EXISTS items;
CREATE TABLE items (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL,
    title TEXT NOT NULL,
    item_type TEXT NOT NULL DEFAULT 'news',
    url TEXT,
    domain TEXT,
    text TEXT,
    points INT DEFAULT 1 CHECK (points >= 1) NOT NULL,
    score INT DEFAULT 0 NOT NULL,
    comment_count INT DEFAULT 0 NOT NULL, 
    -- todo(sql-enums)
    item_category TEXT NOT NULL DEFAULT 'other',
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    dead BOOLEAN DEFAULT false NOT NULL
);
