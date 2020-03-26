create table if not exists cats (
    id serial primary key,
    name varchar not null
);

insert into cats values (1, 'События');
insert into cats values (2, 'Истории успеха');
