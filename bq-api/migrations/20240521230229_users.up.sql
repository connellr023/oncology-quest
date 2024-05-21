CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    can_reset_password BOOLEAN NOT NULL,
    is_admin BOOLEAN NOT NULL,
    salt BIGINT NOT NULL,
    password TEXT NOT NULL,
    login_count INT DEFAULT 0
);