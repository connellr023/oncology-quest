CREATE TABLE domains(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL
);