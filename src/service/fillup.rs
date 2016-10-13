use entity::{Fillup, CreateFillup};
use service::{IntoModel, Page, ServiceConnection, ServiceResult};

pub trait FillupService {
    fn create<T: CreateFillup>(&self, fillup: T) -> ServiceResult<u64>;
    fn by_id(&self, id: i64) -> ServiceResult<Fillup>;
    fn by_user(&self, user_id: i64, page: &Page) -> ServiceResult<Vec<Fillup>>;
    fn by_vehicle(&self, user_id: i64, page: &Page) -> ServiceResult<Vec<Fillup>>;
}

pub struct PgFillupService {
    connection: ServiceConnection
}

impl PgFillupService {
    pub fn new(connection: ServiceConnection) -> PgFillupService {
        PgFillupService {
            connection: connection
        }
    }
}

impl FillupService for PgFillupService {
    fn create<T: CreateFillup>(&self, fillup: T) -> ServiceResult<u64> {
        let sql = include_str!("../../sql/fillup/create.sql");
        Ok(self.connection.execute(sql, &[
            &fillup.user_id(),
            &fillup.vehicle_id(),
            &fillup.cost(),
            &fillup.qty(),
        ])?)
    }
    
    fn by_id(&self, id: i64) -> ServiceResult<Fillup> {
        let sql = include_str!("../../sql/fillup/by_id.sql");
        self.connection.query(sql, &[&id])?.single()
    }

    fn by_user(&self, user_id: i64, page: &Page) -> ServiceResult<Vec<Fillup>> {
        let sql = include_str!("../../sql/vehicle/by_user.sql");
        Ok(self.connection.query(sql, &[&user_id, &page.skip(), &page.take()])?.multiple())
    }

    fn by_vehicle(&self, vehicle_id: i64, page: &Page) -> ServiceResult<Vec<Fillup>> {
        let sql = include_str!("../../sql/vehicle/by_user.sql");
        Ok(self.connection.query(sql, &[&vehicle_id, &page.skip(), &page.take()])?.multiple())
    }
}
