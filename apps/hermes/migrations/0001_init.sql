CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY,
    email: varchar(320) UNIQUE NOT NULL,
);

CREATE TABLE IF NOT EXISTS shortlinks (
    id serial PRIMARY KEY,
    code VARCHAR(16) UNIQUE NOT NULL,
    url VARCHAR(2048) NOT NULL,
);
