create table fillup (
    id bigserial primary key,
    user_id bigserial references "user" (id) not null,
    vehicle_id bigserial references vehicle (id) not null,
    cost bigserial not null,
    qty double precision not null
);

create index fillup_by_user on fillup (user_id);
create index fillup_by_vehicle on fillup (vehicle_id);

insert into fillup (user_id, vehicle_id, cost, qty) values (
    1,
    1,
    10000,
    10.102
);

insert into fillup (user_id, vehicle_id, cost, qty) values (
    2,
    2,
    9000,
    9.685
);
