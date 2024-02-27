-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    auth_token TEXT,
    auth_token_expiration BIGINT,
    reset_password_token TEXT,
    reset_password_token_expiration BIGINT,
    email TEXT NOT NULL DEFAULT '',
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    karma INTEGER DEFAULT 0 CHECK (karma >= 0) NOT NULL,
    about TEXT,
    show_dead BOOLEAN DEFAULT false NOT NULL,
    is_moderator BOOLEAN DEFAULT false NOT NULL,
    shadow_banned BOOLEAN DEFAULT false NOT NULL,
    banned BOOLEAN DEFAULT false NOT NULL
);

