-- Your SQL goes here
drop table if exists user_votes;
drop type if exists vote_type;
drop type if exists user_vote_type;

CREATE TYPE user_vote_type AS ENUM ('Item', 'Comment');

CREATE TABLE user_votes (
    username VARCHAR(255) NOT NULL,
    vote_type USER_VOTE_TYPE NOT NULL,
    content_id UUID NOT NULL,
    parent_item_id UUID,
    upvote BOOLEAN NOT NULL,
    downvote BOOLEAN NOT NULL,
    date TIMESTAMP WITHOUT TIME ZONE NOT NULL,

    CONSTRAINT pk_user_votes PRIMARY KEY (username, content_id, vote_type),
    CONSTRAINT fk_user_votes_users FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE,
    CONSTRAINT fk_user_votes_content_id_items FOREIGN KEY (content_id) REFERENCES items(id) ON DELETE CASCADE,
    CONSTRAINT fk_user_votes_content_id_comments FOREIGN KEY (content_id) REFERENCES comments(id) ON DELETE CASCADE
);

