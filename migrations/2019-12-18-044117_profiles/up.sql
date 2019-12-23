create table if not exists profiles (
    id serial primary key,
    name varchar not null,
    photo varchar not null,
    video varchar not null,
    description text not null,
    published boolean not null default 'F',
    created_at timestamp not null default now()
);
