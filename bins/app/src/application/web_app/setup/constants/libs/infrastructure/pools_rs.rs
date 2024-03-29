pub const CODE: &str = r#"pub use diesel::pg::PgConnection;
use crate::DbPool;
use diesel::r2d2::{ConnectionManager, Pool};


//**************************************************/
//*                    PG
//**************************************************/

// Startup and return Db pool
pub fn get_pg_pool() -> DbPool {
    let mut params =
        config::get_multiple_default(vec![("PG_DB_URL", ""), ("PG_POOL_MAX_SIZE", "8")]);

    let pool_size: u32 = params.pop().unwrap().parse().unwrap();
    let database_url = params.pop().unwrap();
    assert!(!database_url.is_empty(), "PG_DB_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(pool_size)
        .build(manager)
        .unwrap_or_else(|e| panic!("Failed to create postgres db pool: {e}"))
}


"#;