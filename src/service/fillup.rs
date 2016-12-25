use diesel::prelude::*;
use entity::Fillup;
use schema::fillup::dsl::*;
use service::{IntoModel, Paging, ServiceConnection, ServiceResult};

pub struct FillupService {
    connection: ServiceConnection
}

impl FillupService {
    pub fn new(connection: ServiceConnection) -> FillupService {
        FillupService {
            connection: connection
        }
    }

    pub fn by_id(&self, target_id: i64) -> ServiceResult<Fillup> {
        fillup.filter(id.eq(target_id)).limit(1).load(&*self.connection).single()
    }

    pub fn by_user<T: Paging>(&self, target_user_id: i64, page: &T) -> ServiceResult<Vec<Fillup>> {
        fillup.filter(user_id.eq(target_user_id))
            .offset(page.offset())
            .limit(page.limit())
            .load(&*self.connection)
            .multiple()
    }

    pub fn by_vehicle<T: Paging>(&self, target_vehicle_id: i64, page: &T) -> ServiceResult<Vec<Fillup>> {
        fillup.filter(vehicle_id.eq(target_vehicle_id))
            .offset(page.offset())
            .limit(page.limit())
            .load(&*self.connection)
            .multiple()
    }
}
