use schema::user;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub hash: String,
}

#[derive(Insertable)]
#[table_name="user"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub hash: &'a str,
}
