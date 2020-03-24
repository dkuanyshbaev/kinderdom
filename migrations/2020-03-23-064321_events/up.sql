create table if not exists events (
    id serial primary key,
    title varchar not null,
    lead varchar not null,
    cover varchar not null,
    content text not null,
    published boolean not null default 'f',
    created_at timestamp not null default now()
);
