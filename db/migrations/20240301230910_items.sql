-- Add migration script here

DROP TYPE IF EXISTS item_category_enum;
CREATE TYPE item_category_enum as ENUM ('Tweet', 'Blog', 'Paper', 'Other');

DROP TYPE IF EXISTS item_type;
CREATE TYPE item_type as ENUM ('news', 'show', 'ask');

DROP TABLE IF EXISTS items;
CREATE TABLE items (
    id UUID PRIMARY KEY,
    by TEXT NOT NULL,
    title TEXT NOT NULL,
    item_type ITEM_TYPE NOT NULL DEFAULT 'news',
    url TEXT,
    domain TEXT,
    text TEXT,
    points INT DEFAULT 1 CHECK (points >= 1) NOT NULL,
    score INT DEFAULT 0 NOT NULL,
    comment_count INT DEFAULT 0 NOT NULL, 
    -- todo: cat
    item_category ITEM_CATEGORY_ENUM NOT NULL DEFAULT 'Other',
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    dead BOOLEAN DEFAULT false NOT NULL
);
