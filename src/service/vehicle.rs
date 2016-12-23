use diesel::prelude::*;
use entity::Vehicle;
use schema::vehicle::dsl::*;
use service::{IntoModel, Page, ServiceConnection, ServiceResult};

pub struct VehicleService {
    connection: ServiceConnection
}

impl VehicleService {
    pub fn new(connection: ServiceConnection) -> VehicleService {
        VehicleService { connection: connection }
    }
    pub fn by_id(&self, target_id: i64) -> ServiceResult<Vehicle> {
        vehicle.filter(id.eq(target_id)).limit(1).load(&*self.connection).single()
    }

    pub fn by_user(&self, target_user_id: i64, page: &Page) -> ServiceResult<Vec<Vehicle>> {
        vehicle.filter(user_id.eq(target_user_id))
            .offset(page.offset())
            .limit(page.limit())
            .load(&*self.connection)
            .multiple()
    }
}
