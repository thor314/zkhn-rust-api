-- Add migration script here

DROP TYPE IF EXISTS item_category_enum;
CREATE TYPE item_category_enum as ENUM ('tweet', 'blog', 'paper', 'other');

DROP TYPE IF EXISTS item_type;
CREATE TYPE item_type_enum as ENUM ('news', 'show', 'ask');

DROP TABLE IF EXISTS items;
CREATE TABLE items (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL,
    title TEXT NOT NULL,
    item_type ITEM_TYPE_ENUM NOT NULL DEFAULT 'news',
    url TEXT,
    url TEXT,
    domain TEXT,
    text TEXT,
    points INT DEFAULT 1 CHECK (points >= 1) NOT NULL,
    score INT DEFAULT 0 NOT NULL,
    item_category ITEM_CATEGORY_ENUM NOT NULL DEFAULT 'other',
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    dead BOOLEAN DEFAULT false NOT NULL
);
