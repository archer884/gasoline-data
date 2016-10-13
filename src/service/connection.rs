use dotenv::dotenv;
use r2d2_postgres::TlsMode;
use r2d2::{Config, Pool};
use service::*;
use std::env;

pub struct PgConnectionService {
    pool: Pool<ConnectionManager>,
}

impl PgConnectionService {
    pub fn new() -> PgConnectionService {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = Pool::new(
            Config::default(),
            ConnectionManager::new(database_url.as_ref(), TlsMode::None).expect(
                "Unable to create connection manager"
            ),
        ).expect("failed to initialize pool");

        PgConnectionService { pool: pool }
    }
}

pub trait ConnectionService {
    type UserService: UserService;
    type VehicleService: VehicleService;
    type FillupService: FillupService;

    fn users(&mut self) -> Self::UserService;
    fn vehicles(&mut self) -> Self::VehicleService;
    fn fillups(&mut self) -> Self::FillupService;
}

impl ConnectionService for PgConnectionService {
    type UserService = PgUserService;
    type VehicleService = PgVehicleService;
    type FillupService = PgFillupService;

    fn users(&mut self) -> Self::UserService {
        PgUserService::new(self.pool.get().expect("unable to get connection"))
    }

    fn vehicles(&mut self) -> Self::VehicleService {
        PgVehicleService::new(self.pool.get().expect("unable to get connection"))
    }

    fn fillups(&mut self) -> Self::FillupService {
        PgFillupService::new(self.pool.get().expect("unable to get connection"))
    }
}
