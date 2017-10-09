CREATE TABLE authors (
    id VARCHAR PRIMARY KEY UNIQUE NOT NULL
);

CREATE TABLE quotes (
    id SERIAL PRIMARY KEY,
    message_id VARCHAR UNIQUE NOT NULL,
    quote VARCHAR NOT NULL,
    created_by_id VARCHAR REFERENCES authors(id) NOT NULL,
    quoted_by_id VARCHAR REFERENCES authors(id) NOT NULL,
    guild_id VARCHAR NOT NULL
);