create table if not exists articles (
    id serial primary key,
    title varchar not null,
    image varchar not null,
    content text not null,
    published boolean not null default 'f',
    created_at timestamp not null default now()
);
