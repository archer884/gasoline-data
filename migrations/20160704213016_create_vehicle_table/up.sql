create table vehicle (
    id bigserial primary key,
    user_id bigserial references "user" (id) not null, 
    name varchar(240) not null,
    description varchar(240),
    image varchar(240)
);

create index vehicle_by_user on vehicle (user_id);

insert into vehicle (user_id, name, description) values (
    1,
    'CBR1000RR',
    'Badass fucking bike'
);

insert into vehicle (user_id, name, description) values (
    2,
    'YZF-R1',
    'Other badass fucking bike'
);
