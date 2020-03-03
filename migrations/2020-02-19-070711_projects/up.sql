create table if not exists projects (
    id serial primary key,
    name varchar not null,
    image varchar not null,
    description text not null,
    published boolean not null default 'f',
    created_at timestamp not null default now()
);
