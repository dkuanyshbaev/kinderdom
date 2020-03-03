create table if not exists events (
    id serial primary key,
    name varchar not null,
    image varchar not null,
    needed int not null,
    collected int not null,
    description text not null,
    is_vital boolean not null default 'f',
    published boolean not null default 'f',
    created_at timestamp not null default now()
);
