-- Add up migration script here

drop table if exists ulid;
drop table if exists user_votes;
drop type if exists item_or_comment_enum;
drop type if exists vote_state_enum;

CREATE TYPE vote_state_enum AS ENUM ('upvote', 'downvote', 'none');

CREATE TYPE item_or_comment_enum AS ENUM ('item', 'comment');

CREATE DOMAIN ulid AS VARCHAR(26);

CREATE TABLE user_votes (
    id UUID PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    vote_type ITEM_OR_COMMENT_ENUM NOT NULL,
    content_id ULID NOT NULL,
    parent_item_id ULID, 
    vote_state VOTE_STATE_ENUM NOT NULL DEFAULT 'upvote',
    created TIMESTAMP WITH TIME ZONE NOT NULL

    -- CONSTRAINT pk_user_votes PRIMARY KEY (username, content_id, vote_type),
    -- CONSTRAINT fk_user_votes_users FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE,
    -- CONSTRAINT fk_user_votes_content_id_items FOREIGN KEY (content_id) REFERENCES items(id) ON DELETE CASCADE,
    -- CONSTRAINT fk_user_votes_content_id_comments FOREIGN KEY (content_id) REFERENCES comments(id) ON DELETE CASCADE
);



-- -- create a query that alters user_votes by setting id the primary key, updating it away from username
-- -- Add a new column 'id' of type SERIAL
-- ALTER TABLE user_votes ADD COLUMN id UUID;

-- -- Set the new 'id' column as the primary key
-- ALTER TABLE user_votes ADD PRIMARY KEY (id);

-- -- If you want to remove 'username' as the primary key
-- -- ALTER TABLE user_votes DROP CONSTRAINT user_votes_pkey;
-- -- ALTER TABLE user_votes DROP COLUMN username;