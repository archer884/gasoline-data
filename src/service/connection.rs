use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2::{Config, Pool};
use service::{ConnectionManager, ServiceConnection, UserService, VehicleService, FillupService};
use std::env;

pub struct ConnectionService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ConnectionService {
    pub fn new() -> ConnectionService {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = Pool::new(
            Config::default(),
            ConnectionManager::new(database_url.as_ref()),
        ).expect("failed to initialize pool");

        ConnectionService { pool: pool }
    }

    pub fn users(&self) -> UserService {
        UserService::new(self.get_connection())
    }

    pub fn vehicles(&self) -> VehicleService {
        VehicleService::new(self.get_connection())
    }

    pub fn fillups(&self) -> FillupService {
        FillupService::new(self.get_connection())
    }

    fn get_connection(&self) -> ServiceConnection {
        self.pool.get().expect("unable to get connection")
    }
}
