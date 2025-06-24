CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY,
    email varchar(320) NOT NULL,
    UNIQUE (email)
);

CREATE TABLE IF NOT EXISTS shortlinks (
    id serial PRIMARY KEY,
    code varchar(16) NOT NULL,
    url varchar(2048) NOT NULL,
    UNIQUE (code)
);
