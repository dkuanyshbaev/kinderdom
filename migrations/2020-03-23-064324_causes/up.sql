create table if not exists causes (
    id serial primary key,
    name varchar not null,
    image varchar not null,
    needed int not null,
    collected int not null,
    location varchar not null,
    organisation varchar not null,
    description text not null,
    vital boolean not null default 'f',
    published boolean not null default 'f',
    created_at timestamp not null default now()
);
