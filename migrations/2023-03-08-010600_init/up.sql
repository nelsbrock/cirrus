CREATE TABLE users (
    name TEXT PRIMARY KEY NOT NULL,
    password_hash TEXT
);

CREATE TABLE public_shares (
    id TEXT PRIMARY KEY NOT NULL,
    file_path TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
