use postgres::rows::Row;

/// Cost is stored as an integral value by first multiplying the 
/// decimal value of the total fuel cost by 1000. 
#[derive(Debug)]
pub struct Fillup {
    pub id: i64,
    pub user_id: i64,
    pub vehicle_id: i64,
    pub cost: i64,
    pub qty: f64,
}

pub trait CreateFillup {
    fn user_id(&self) -> i64;
    fn vehicle_id(&self) -> i64;
    fn cost(&self) -> i64;
    fn qty(&self) -> f64;
}

impl CreateFillup for (i64, i64, i64, f64) {
    fn user_id(&self) -> i64 {
        self.0
    }

    fn vehicle_id(&self) -> i64 {
        self.1
    }

    fn cost(&self) -> i64 {
        self.2
    }

    fn qty(&self) -> f64 {
        self.3
    }
}

impl<'a> From<Row<'a>> for Fillup {
    fn from(row: Row) -> Self {
        Fillup {
            id: row.get(0),
            user_id: row.get(1),
            vehicle_id: row.get(2),
            cost: row.get(3),
            qty: row.get(4),
        }
    }
}
