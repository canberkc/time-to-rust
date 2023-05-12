create table player
(
    id            bytea   not null
        primary key,
    name          varchar(255),
    lastname       varchar(255),
    team        varchar(255),
    nationality   varchar(255)
);

alter table public.player
    owner to test;