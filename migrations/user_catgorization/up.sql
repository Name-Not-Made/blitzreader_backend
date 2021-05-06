CREATE TABLE IF NOT EXISTS users (
    id serial primary key,
    username text not null unique,
    email text not null unique,
    password text not null,
    created_on timestamp not null default now(),
    last_login timestamp,
    timezone text not null,
    email_verified boolean not null default 'f'
);