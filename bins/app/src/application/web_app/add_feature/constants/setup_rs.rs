pub fn get_setup_code(pascal_feature_name: &str, feature_name: &str) -> String {
    let mut template =  r#"use super::{domain::{0}, http::*, infrastructure::{PgRepository, PgService}};
use infrastructure::db::Pg;
use actix_web::web;
use std::sync::Arc;

pub fn routes(pg_pool: Arc<Pg>, cfg: &mut web::ServiceConfig) {
    let service = {0} {
        service: PgService {
            pg_pool: pg_pool.clone(),
        },
        repository: PgRepository { pg_pool },
    };
    
    cfg.app_data(web::Data::new(service));

    cfg.service(
        web::resource("/api/{1}/{version}")
            .route(web::get().to(handle_{1}::<{0}<PgRepository, PgService>>))
    );
}
"#;
    
    let binding = template.replace("{0}", pascal_feature_name);
    template = &binding;
    template.replace("{1}", feature_name)
}