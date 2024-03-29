pub mod application;

#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main(){
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    config::initialize().await;

    application::configure().await;
}