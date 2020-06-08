create table if not exists reports (
    id serial primary key,
    url varchar not null,
    description varchar not null,
    created_at timestamp not null default now()
);
