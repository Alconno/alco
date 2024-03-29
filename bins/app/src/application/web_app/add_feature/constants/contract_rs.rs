pub fn get_contract_code(feature_name: &str) -> String{
    let template = r#"use async_trait::async_trait;

#[async_trait]
pub trait {0}Contract {

}

// getters
#[async_trait]
pub trait PgRepositoryContract {

}


// setters
#[async_trait]
pub trait PgServiceContract {

}
    "#;

    template.replace("{0}", feature_name)
}