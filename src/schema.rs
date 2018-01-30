table! {
    fillup (id) {
        id -> Int8,
        user_id -> Int8,
        vehicle_id -> Int8,
        cost -> Int8,
        qty -> Float8,
    }
}

table! {
    user (id) {
        id -> Int8,
        username -> Varchar,
        hash -> Bytea,
    }
}

table! {
    vehicle (id) {
        id -> Int8,
        user_id -> Int8,
        name -> Varchar,
        description -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
    }
}

joinable!(fillup -> user (user_id));
joinable!(fillup -> vehicle (vehicle_id));
joinable!(vehicle -> user (user_id));

allow_tables_to_appear_in_same_query!(
    fillup,
    user,
    vehicle,
);
