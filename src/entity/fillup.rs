use schema::fillup;

/// Cost is stored as an integral value by first multiplying the 
/// decimal value of the total fuel cost by 1000. 
#[derive(Debug, Queryable)]
pub struct Fillup {
    pub id: i64,
    pub user_id: i64,
    pub vehicle_id: i64,
    pub cost: i64,
    pub qty: f64,
}

#[derive(Insertable)]
#[table_name="fillup"]
pub struct NewFillup {
    pub user_id: i64,
    pub vehicle_id: i64,
    pub cost: i64,
    pub qty: f64,
}
