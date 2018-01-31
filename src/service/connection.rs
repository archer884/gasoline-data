use diesel::PgConnection;
use dotenv::dotenv;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;
use service::{ServiceConnection, UserService, VehicleService, FillupService};
use std::env;

pub struct ConnectionService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Default for ConnectionService {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionService {
    pub fn new() -> ConnectionService {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create connection pool.");

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
