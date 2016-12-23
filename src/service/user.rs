use diesel::prelude::*;
use schema::user::dsl::*;
use entity::User;
use service::{IntoModel, ServiceConnection, ServiceResult};

pub struct UserService {
    connection: ServiceConnection
}

impl UserService {
    pub fn new(connection: ServiceConnection) -> UserService {
        UserService { connection: connection }
    }
    
    pub fn by_id(&self, target: i64) -> ServiceResult<User> {
        user.filter(id.eq(target)).limit(1).load(&*self.connection).single()
    }

    pub fn by_username(&self, target: &str) -> ServiceResult<User> {
        user.filter(username.eq(target)).limit(1).load(&*self.connection).single()
    }
}
