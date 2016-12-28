use schema::vehicle;

#[derive(Debug, Queryable)]
pub struct Vehicle {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Insertable)]
#[table_name="vehicle"]
pub struct NewVehicle<'a> {
    pub user_id: i64,
    pub name: &'a str,
    pub description: Option<&'a str>,
}
