pub const CODE: &str = r#"use crate::db::Pg;
pub use diesel::pg::PgConnection;
use std::sync::Arc;
use crate::{DbPool, DbConnection};


#[derive(Clone)]
pub struct Pools {
    pub pg: DbPool,
}

#[derive(Clone)]
pub struct AppState {
    pub pools: Arc<Pools>,
}

impl AppState {
    pub fn new(pool: DbPool) -> Self {
        AppState {
            pools: Arc::new(Pools { pg: pool }),
        }
    }

    pub fn pg_connection(&self) -> DbConnection {
        match self.pools.pg.get() {
            Ok(connection) => connection,
            Err(e) => panic!("State: {e}"),
        }
    }
}

// Initialization
pub fn initialize() -> (AppState, Arc<Pg>) {
    let pg_pool = crate::pools::get_pg_pool();
    let pg = Arc::new(Pg::new(pg_pool.clone()));  

    (AppState::new(pg_pool), pg)
}

"#;