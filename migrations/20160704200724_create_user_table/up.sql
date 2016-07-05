create table "user" (
    id bigserial primary key,
    username varchar(240) not null,
    hash varchar(240) not null
);

create unique index user_by_username on "user" (username);

insert into "user" (username, hash) values ('archer884', 'this is not a hash');
insert into "user" (username, hash) values ('savage884', 'this is not a hash');
