-- Your SQL goes here

DROP TABLE IF EXISTS items;

CREATE TYPE item_category_enum as ENUM ('Tweet', 'Blog', 'Paper', 'Other');

CREATE TABLE items (
    id UUID PRIMARY KEY,
    by TEXT NOT NULL,
    title TEXT NOT NULL,
    item_type TEXT NOT NULL CHECK (item_type IN ('news', 'show', 'ask')),
    url TEXT,
    domain TEXT,
    text TEXT,
    points INT DEFAULT 1 CHECK (points >= 1) NOT NULL,
    score INT DEFAULT 0 NOT NULL,
    comment_count INT DEFAULT 0 NOT NULL, 
    -- todo: cat
    category ITEM_CATEGORY_ENUM NOT NULL DEFAULT 'Other',
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    dead BOOLEAN DEFAULT false NOT NULL
);
