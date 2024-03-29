pub const CODE: &str = r#"use actix_web::HttpRequest;
use error::Error;
use std::str::FromStr;


/// Parse given parameter easily from the request path string
pub fn part_from_path<T: FromStr>(req: &HttpRequest, name: &str) -> Result<T, Error> {
    let value: T = match req.match_info().get(name) {
        Some(val) => match val.parse() {
            Ok(v) => v,
            Err(_) => return Err(Error::BadRequest(format!("path_attribute_missing:{name}"))),
        },
        None => return Err(Error::BadRequest(format!("path_attribute_missing:{name}"))),
    };

    Ok(value)
}

/// Get api version from the route path
pub fn get_api_version(req: &HttpRequest) -> String {
    match req.match_info().get("version") {
        Some(value) => value.parse().unwrap_or_else(|_| "v1".to_string()),
        None => "v1".to_string(),
    }
}
"#;