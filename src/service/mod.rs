use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use r2d2_diesel::ConnectionManager;
use r2d2::PooledConnection;
use std::error::Error;
use std::fmt;

mod connection;
mod fillup;
mod page;
mod user;
mod vehicle;

pub use service::connection::ConnectionService;
pub use service::fillup::FillupService;
pub use service::page::Page;
pub use service::user::UserService;
pub use service::vehicle::VehicleService;

pub type ServiceConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Debug)]
pub enum ServiceError {
    NotFound,
    Database(Box<Error + Send>),
}

impl From<DieselError> for ServiceError {
    fn from(error: DieselError) -> ServiceError {
        ServiceError::Database(box error)
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ServiceError::NotFound => write!(f, "Not found"),
            &ServiceError::Database(ref e) => write!(f, "Database failure: {:?}", e),
        }
    }
}

impl Error for ServiceError {
    fn description(&self) -> &str {
        match *self {
            ServiceError::NotFound => "Entity not found by identifier",
            ServiceError::Database(_) => "Underlying database failure",
        }
    }
}

trait IntoModel {
    type Model;
    fn single(self) -> ServiceResult<Self::Model>;
    fn multiple(self) -> ServiceResult<Vec<Self::Model>>;
}

impl<T> IntoModel for Result<Vec<T>, DieselError> {
    type Model = T;

    fn single(self) -> ServiceResult<Self::Model> {
        self?.pop().ok_or(ServiceError::NotFound)
    }

    fn multiple(self) -> ServiceResult<Vec<Self::Model>> {
        Ok(self?)
    }
}
