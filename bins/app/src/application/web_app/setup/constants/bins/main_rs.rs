pub const CODE: &str = r#"pub mod web;
pub mod application;

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main(){
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    config::initialize().await;

    let (state, pg) = infrastructure::state::initialize();

    web::start_server(state, pg).await.expect("Error while starting/running http server");
}
"#;