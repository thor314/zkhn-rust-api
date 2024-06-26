-- Add up migration script here

DROP TABLE IF EXISTS users;
CREATE TABLE users (
    username TEXT PRIMARY KEY,
    password_hash TEXT NOT NULL,
    reset_password_token TEXT,
    reset_password_token_expiration TIMESTAMP WITH TIME ZONE,
    email TEXT,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    karma INTEGER DEFAULT 0 CHECK (karma >= 0) NOT NULL,
    about TEXT,
    show_dead BOOLEAN DEFAULT false NOT NULL,
    is_moderator BOOLEAN DEFAULT false NOT NULL,
    shadow_banned BOOLEAN DEFAULT false NOT NULL,
    banned BOOLEAN DEFAULT false NOT NULL
);

