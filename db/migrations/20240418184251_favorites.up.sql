drop table if exists user_favorites;

CREATE TABLE user_favorites (
    id UUID PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    item_type VARCHAR(50) NOT NULL,
    item_id UUID NOT NULL,
    date TIMESTAMP WITH TIME ZONE NOT NULL

    -- CONSTRAINT pk_user_favorites PRIMARY KEY (username, item_type, item_id),
    -- CONSTRAINT fk_user_favorites_users FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE,
    -- CONSTRAINT fk_user_favorites_items FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE
);
