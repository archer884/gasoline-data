use entity::{User, CreateUser};
use service::{IntoModel, ServiceConnection, ServiceResult};

pub trait UserService {
    fn by_id(&self, id: i64) -> ServiceResult<User>;
    fn by_username(&self, username: &str) -> ServiceResult<User>;
}

pub struct PgUserService {
    connection: ServiceConnection
}

impl PgUserService {
    pub fn new(connection: ServiceConnection) -> PgUserService {
        PgUserService {
            connection: connection
        }
    }
}

impl UserService for PgUserService {
    fn by_id(&self, id: i64) -> ServiceResult<User> {
        let sql = include_str!("../../sql/user/by_id.sql");
        self.connection.query(sql, &[&id])?.single()
    }

    fn by_username(&self, username: &str) -> ServiceResult<User> {
        let sql = include_str!("../../sql/user/by_username.sql");
        self.connection.query(sql, &[&username])?.single()
    }
}
