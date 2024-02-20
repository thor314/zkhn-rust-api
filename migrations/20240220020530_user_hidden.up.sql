-- Add up migration script here
CREATE TABLE user_hidden (
    username VARCHAR(255) NOT NULL,
    item_id UUID NOT NULL,
    date TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    item_creation_date TIMESTAMP WITHOUT TIME ZONE NOT NULL,

    CONSTRAINT pk_user_hidden PRIMARY KEY (username, item_id),
    CONSTRAINT fk_user_hidden_users FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE,
    CONSTRAINT fk_user_hidden_items FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE
);
