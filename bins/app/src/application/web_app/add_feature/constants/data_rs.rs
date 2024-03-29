pub fn get_data_code(feature_name: &str) -> String {
    let template =  r#"use validr::Validation;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request{0}Data {
    
}

#[derive(Clone, Debug)]
pub struct {0}Data {
    
}

impl Validation for Request{0}Data {
    fn modifiers(&self) -> Vec<validr::Modifier<Self>> {
        vec![]
    }

    fn rules(&self) -> Vec<validr::Rule<Self>> {
        vec![]
    }
}

impl From<Request{0}Data> for {0}Data {
    fn from(source: Request{0}Data) -> Self {
        Self {

        }
    }
}
"#;
    
    template.replace("{0}", feature_name)
}