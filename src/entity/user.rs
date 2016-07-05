use postgres::rows::Row;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub hash: String,
}

pub trait CreateUser {
    fn username(&self) -> &str;
    fn hash(&self) -> &str;
}

impl<'a, T: AsRef<str> + 'a> CreateUser for (T, T) {
    fn username(&self) -> &str {
        self.0.as_ref()
    }

    fn hash(&self) -> &str {
        self.1.as_ref()
    }
}

impl<'a> From<Row<'a>> for User {
    fn from(row: Row) -> Self {
        User {
            id: row.get(0),
            username: row.get(1),
            hash: row.get(2),
        }
    }
}