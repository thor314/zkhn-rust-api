-- Add up migration script here
CREATE TABLE user_favorites (
    username VARCHAR(255) NOT NULL,
    item_type VARCHAR(50) NOT NULL,
    item_id UUID NOT NULL,
    date TIMESTAMP WITHOUT TIME ZONE NOT NULL,

    CONSTRAINT pk_user_favorites PRIMARY KEY (username, item_type, item_id),
    CONSTRAINT fk_user_favorites_users FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE,
    CONSTRAINT fk_user_favorites_items FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE
);

