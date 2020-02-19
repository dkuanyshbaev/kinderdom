create table if not exists events (
    id serial primary key,
    published boolean not null default 'f',
    created_at timestamp not null default now()
);
