pub const CODE: &str = r#"use actix_web::{ App, HttpServer};
pub use diesel::pg::PgConnection;
use infrastructure::state::AppState;
use std::sync::Arc;
use infrastructure::db::Pg;

/// Start the server
#[cfg(not(tarpaulin_include))]
pub async fn start_server(state: AppState, pg: Arc<Pg>) -> std::io::Result<()> {

    let address = format!(
        "{0}:{1}",
        config::get_default("LISTEN_ADDRESS", "0.0.0.0"),
        config::get_default("LISTEN_PORT", "4554")
    );


    HttpServer::new(move || {
        App::new()
            .app_data::<AppState>(state.clone())
            .configure(|cfg| crate::application::configure(
                Arc::clone(&pg),
                cfg
            ))
    })
    .workers(10) 
    .bind(address)
    .expect("Unable to bind server")
    .run()
    .await
}
"#;