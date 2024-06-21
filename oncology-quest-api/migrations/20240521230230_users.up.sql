CREATE TABLE IF NOT EXISTS users(
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    name TEXT NOT NULL,
    password_reset_timestamp TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    password_reset_token TEXT DEFAULT NULL,
    is_admin BOOLEAN NOT NULL,
    salt BIGINT NOT NULL,
    password TEXT NOT NULL,
    login_count INT DEFAULT 0 NOT NULL
);