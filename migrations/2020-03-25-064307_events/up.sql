create table if not exists events (
    id serial primary key,
    title varchar not null,
    lead varchar not null,
    cover varchar not null,
    content text not null,
    published boolean not null default 'f',
    cat_id integer not null default 1 references cats on delete set default,
    created_at timestamp not null default now()
);
