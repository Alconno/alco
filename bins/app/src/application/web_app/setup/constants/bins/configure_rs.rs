pub const CODE: &str = r#"use actix_web::{ web, HttpResponse};
use infrastructure::db::Pg;
use std::sync::Arc;

pub fn configure(pg_pool: Arc<Pg>, cfg: &mut web::ServiceConfig) {

    cfg.route(
        "/_health",
        web::get().to(|| async { HttpResponse::Ok().body("I am healthy!")})
    );


}


"#;