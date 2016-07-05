use std::env;
use dotenv::dotenv;
use r2d2::{Config, Pool};
use r2d2_postgres::SslMode;
use service::{ConnectionManager, UserService, PgUserService};

pub struct PgConnectionService {
    pool: Pool<ConnectionManager>,
}

impl PgConnectionService {
    pub fn new() -> PgConnectionService {
        dotenv().ok();
        
        let database_url = env::var("DATABASE_URL").expect(
            "DATABASE_URL must be set"
        );
        
        let pool = Pool::new(
            Config::default(),
            ConnectionManager::new(database_url.as_ref(), SslMode::None).expect(
                "Unable to create connection manager"
            ),
        ).expect("failed to initialize pool");
        
        PgConnectionService {
            pool: pool,
        }
    }
}

pub trait ConnectionService {
    type UserService: UserService;
    
    fn users(&mut self) -> Self::UserService; 
}

impl ConnectionService for PgConnectionService {
    type UserService = PgUserService;
    
    fn users(&mut self) -> Self::UserService {
        PgUserService::new(self.pool.get().expect("unable to get connection"))
    }
}