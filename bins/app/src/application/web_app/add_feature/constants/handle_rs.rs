pub fn get_handle_code(pascal_feature_name: &str, feature_name: &str) -> String {
    let mut template =  r#"use super::super::contract::{0}Contract;
use actix_web::{web, HttpRequest, HttpResponse};
use error::Error;
use support::helpers::http::get_api_version;

pub async fn handle_{1}<T: {0}Contract>(
    req: HttpRequest,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    match get_api_version(&req).as_ref() {
        "v1" => handle_{1}_v1(service).await,
        _ => Err(Error::NotFound),
    }
}

pub async fn handle_{1}_v1<T: {0}Contract>(
    service: web::Data<T>
) -> Result<HttpResponse, Error> {

    println!("Not yet implemented!");

    Ok(HttpResponse::Ok().into())
}
"#;
    
    let binding = template.replace("{0}", pascal_feature_name);
    template = &binding;
    template.replace("{1}", feature_name)
}