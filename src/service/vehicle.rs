use entity::{Vehicle, CreateVehicle};
use service::{IntoModel, Page, ServiceConnection, ServiceResult};

pub trait VehicleService {
    fn by_id(&self, id: i64) -> ServiceResult<Vehicle>;
    fn by_user(&self, id: i64, page: &Page) -> ServiceResult<Vec<Vehicle>>;
}

pub struct PgVehicleService {
    connection: ServiceConnection
}

impl PgVehicleService {
    pub fn new(connection: ServiceConnection) -> PgVehicleService {
        PgVehicleService {
            connection: connection
        }
    }
}

impl VehicleService for PgVehicleService {
    fn by_id(&self, id: i64) -> ServiceResult<Vehicle> {
        let sql = include_str!("../../sql/vehicle/by_id.sql");
        self.connection.query(sql, &[&id])?.single()
    }

    fn by_user(&self, user_id: i64, page: &Page) -> ServiceResult<Vec<Vehicle>> {
        let sql = include_str!("../../sql/vehicle/by_user.sql");
        Ok(self.connection.query(sql, &[&user_id, &page.skip(), &page.take()])?.multiple())
    }
}
