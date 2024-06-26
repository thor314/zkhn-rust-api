-- Add up migration script here
DROP TYPE IF EXISTS item_category_enum;
CREATE TYPE item_category_enum as ENUM ('tweet', 'blog', 'paper', 'other');

DROP TYPE IF EXISTS item_type_enum;
CREATE TYPE item_type_enum as ENUM ('news', 'show', 'ask');

DROP TABLE IF EXISTS items;
CREATE TABLE items (
    id VARCHAR(26) PRIMARY KEY, 
    username TEXT NOT NULL,
    title TEXT NOT NULL,
    item_type ITEM_TYPE_ENUM NOT NULL DEFAULT 'news',
    url TEXT,
    domain TEXT,
    comment_count INT DEFAULT 0 NOT NULL,
    text TEXT,
    points INT DEFAULT 1 NOT NULL,
    score INT DEFAULT 0 NOT NULL,
    item_category ITEM_CATEGORY_ENUM NOT NULL DEFAULT 'other',
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    dead BOOLEAN DEFAULT false NOT NULL
);