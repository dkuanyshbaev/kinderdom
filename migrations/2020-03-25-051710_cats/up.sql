create table if not exists cats (
    id serial primary key,
    name varchar not null,
    en boolean not null default 'f'
);

insert into cats values (1, 'События', 'f');
insert into cats values (2, 'Истории успеха', 'f');
insert into cats values (3, 'Events', 't');
insert into cats values (4, 'Success stories', 't');
