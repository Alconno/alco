pub const CODE: &str = r#"#![allow(proc_macro_derive_resolution_fallback)]
#![recursion_limit = "256"]

#[allow(unused_imports)]
#[macro_use]
pub extern crate diesel;

pub mod schema;
pub mod db;
pub mod state;
pub mod pools;
pub use pools::*;

pub use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;
"#;