use postgres::rows::Row;

#[derive(Debug)]
pub struct Vehicle {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub desc: String,
    pub image: Option<String>,
}

pub trait CreateVehicle {
    fn user_id(&self) -> i64;
    fn name(&self) -> &str;
    fn desc(&self) -> &str;
    fn image(&self) -> Option<&str>;
}

impl<'a, T: AsRef<str> + 'a> CreateVehicle for (i64, T, T) {
    fn user_id(&self) -> i64 {
        self.0
    }
    
    fn name(&self) -> &str {
        self.1.as_ref()
    }

    fn desc(&self) -> &str {
        self.2.as_ref()
    }

    fn image(&self) -> Option<&str> {
        None
    }
}

impl<'a> From<Row<'a>> for Vehicle {
    fn from(row: Row) -> Self {
        Vehicle {
            id: row.get(0),
            user_id: row.get(1),
            name: row.get(2),
            desc: row.get(3),
            image: row.get(4),
        }
    }
}
