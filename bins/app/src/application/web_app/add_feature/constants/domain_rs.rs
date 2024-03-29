pub fn get_domain_code(feature_name: &str) -> String {
    let template =  r#"use super::contract::{{0}Contract, PgRepositoryContract, PgServiceContract};
use async_trait::async_trait;

pub struct {0}<A: PgRepositoryContract, B: PgServiceContract> {
    pub repository: A,
    pub service: B,
}

 
#[async_trait]
impl<A, B> {0}Contract for {0}<A, B>
where
    A: PgRepositoryContract + Sync + Send,
    B: PgServiceContract + Sync + Send,
{

}"#;
    
    template.replace("{0}", feature_name)
}