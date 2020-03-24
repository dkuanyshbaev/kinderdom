create table if not exists donors (
    id serial primary key,
    name varchar not null,
    photo varchar not null,
    position varchar not null,
    quote text not null,
    created_at timestamp not null default now()
);
