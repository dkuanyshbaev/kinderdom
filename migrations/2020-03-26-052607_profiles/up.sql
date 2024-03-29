create table if not exists profiles (
    id serial primary key,
    name varchar not null,
    photo varchar not null,
    description text not null,
    published boolean not null default 'f',
    en boolean not null default 'f',
    created_at timestamp not null default now()
);
