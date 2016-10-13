use postgres::error::Error as PgError;
use postgres::rows::{Row, Rows};
use r2d2_postgres;
use r2d2::PooledConnection;
use std::error::Error;
use std::fmt;

mod connection;
mod fillup;
mod page;
mod user;
mod vehicle;

pub use service::connection::{ConnectionService, PgConnectionService};
pub use service::fillup::{FillupService, PgFillupService};
pub use service::page::Page;
pub use service::user::{UserService, PgUserService};
pub use service::vehicle::{VehicleService, PgVehicleService};

pub type ConnectionManager = r2d2_postgres::PostgresConnectionManager;
pub type ServiceConnection = PooledConnection<ConnectionManager>;
pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Debug)]
pub enum ServiceError {
    NotFound,
    BadSchema(Box<Error + Send>),
    DatabaseFailure(Box<Error + Send>),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ServiceError::NotFound => write!(f, "Not found"),
            &ServiceError::BadSchema(ref e) => write!(f, "Unable to convert pg type to rust type: {}", e),
            &ServiceError::DatabaseFailure(ref e) => write!(f, "Database failure: {:?}", e),
        }
    }
}

impl From<PgError> for ServiceError {
    fn from(error: PgError) -> Self {
        match error {
            PgError::Db(e) => ServiceError::DatabaseFailure(e),
            PgError::Io(e) => ServiceError::DatabaseFailure(box e),
            PgError::Conversion(e) => ServiceError::BadSchema(e),
        }
    }
}

impl Error for ServiceError {
    fn description(&self) -> &str {
        match *self {
            ServiceError::NotFound => "Entity not found by identifier",
            ServiceError::BadSchema(_) => "Unable to convert from db schema to entity",
            ServiceError::DatabaseFailure(_) => "Underlying database failure",
        }
    }
}

pub trait IntoModel<'a> {
    fn id(&'a self) -> ServiceResult<i64>;
    fn single<T: From<Row<'a>>>(&'a self) -> ServiceResult<T>;
    fn multiple<T: From<Row<'a>>>(&'a self) -> Vec<T>;
}

impl<'a> IntoModel<'a> for Rows<'a> {
    fn id(&'a self) -> ServiceResult<i64> {
        let row = self.iter().next().unwrap();
        Ok(row.get(0))
    }
    
    fn single<T: From<Row<'a>>>(&'a self) -> ServiceResult<T> {
        self.iter().map(Row::into).next().ok_or(ServiceError::NotFound)
    }
    
    fn multiple<T: From<Row<'a>>>(&'a self) -> Vec<T> {
        self.iter().map(Row::into).collect()
    }
}