create table if not exists docs (
    id serial primary key,
    file_name varchar not null,
    description text not null,
    published boolean not null default 'f',
    created_at timestamp not null default now()
);