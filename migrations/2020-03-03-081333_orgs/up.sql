create table if not exists orgs (
    id serial primary key,
    name varchar not null,
    logo varchar not null,
    description text not null,
    published boolean not null default 'f',
    created_at timestamp not null default now()
);
