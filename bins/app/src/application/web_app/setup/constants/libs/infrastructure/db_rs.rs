pub const CODE: &str = r#"pub use diesel::pg::PgConnection;
use diesel::Connection;
use std::env;
pub use r2d2_redis::redis::{Client, ConnectionAddr, ConnectionInfo, IntoConnectionInfo};
use crate::{DbPool, DbConnection};

/// Struct to hold our postgres pool with some integrated commands
#[derive(Clone)]
pub struct Pg {
    pool: DbPool,
}

impl Pg {
    /// Create new instance of this struct with integrated pool
    pub fn new(pool: DbPool) -> Self {
        Pg { pool }
    }

    /// Get connection from the pool
    pub fn connection(&self) -> Result<DbConnection, error::Error> {
        self.pool.get().map_err(error::Error::from)
    }

    /// Staticly generates completely new independent connection
    /// to use somewhere where pool cannot be passed.
    pub fn single_connection() -> PgConnection {
        let database_url = env::var("PG_DB_URL").expect("PG_DB_URL must be set");

        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url))
    }
}
"#;