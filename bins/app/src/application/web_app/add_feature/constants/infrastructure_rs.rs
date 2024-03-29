pub const INFRASTRUCTURE: &str = r#"use super::contract::{PgRepositoryContract, PgServiceContract};
use async_trait::async_trait;
use infrastructure::db::Pg;
use std::sync::Arc;



// Getters
pub struct PgRepository {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
  
}



// Setters
pub struct PgService {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgServiceContract for PgService {
    
}
"#;