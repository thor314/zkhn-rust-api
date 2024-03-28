-- Add up migration script here

-- let users email column be null
-- remove the NOT NULL constraint and the default value
-- ALTER TABLE users ALTER COLUMN email DROP NOT NULL;
-- ALTER TABLE users ALTER COLUMN email DROP DEFAULT;
-- set empty strings to NULL
-- UPDATE users SET email = NULL WHERE email = '';

-- SELECT pid, usename, application_name, client_addr, state, query_start, query 
-- FROM pg_stat_activity 
-- WHERE datname = 'tk-shuttle-zkhn-rust-api2';

-- SELECT pg_terminate_backend(pg_stat_activity.pid) 
-- FROM pg_stat_activity 
-- WHERE pg_stat_activity.datname = 'tk-shuttle-zkhn-rust-api2' 
-- AND pid <> pg_backend_pid();

DROP TABLE IF EXISTS users;
CREATE TABLE users (
    username TEXT PRIMARY KEY,
    password_hash TEXT NOT NULL,
    auth_token TEXT,
    auth_token_expiration TIMESTAMP WITH TIME ZONE,
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

