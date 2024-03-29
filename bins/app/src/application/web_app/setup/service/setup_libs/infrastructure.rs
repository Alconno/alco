use error::Error;
use tokio::{fs::{self, File}, io::AsyncWriteExt};

use crate::application::web_app::setup::constants::libs::infrastructure::*;

pub async fn setup_libs_infrastructure(project_name: String) -> Result<(), Error>{
    let mut current_dir = project_name;
    // Enter libs dir
    current_dir.push_str("/libs");

    current_dir.push_str("/infrastructure");
    fs::create_dir(&current_dir).await?;

    // Create Cargo toml
    let mut file = File::create(format!("{}/Cargo.toml", current_dir)).await?;
    file.write_all(cargo_toml::CODE.as_bytes()).await?;

    // src folder
    current_dir.push_str("/src");
    fs::create_dir(&current_dir).await?;

    // Create db.rs
    let mut file = File::create(format!("{}/db.rs", current_dir)).await?;
    file.write_all(db_rs::CODE.as_bytes()).await?;

    // Create lib.rs
    let mut file = File::create(format!("{}/lib.rs", current_dir)).await?;
    file.write_all(lib_rs::CODE.as_bytes()).await?;

    // Create pools.rs
    let mut file = File::create(format!("{}/pools.rs", current_dir)).await?;
    file.write_all(pools_rs::CODE.as_bytes()).await?;
    
    // Create schema.rs
    let mut file = File::create(format!("{}/schema.rs", current_dir)).await?;
    file.write_all(schema_rs::CODE.as_bytes()).await?;
    
    // Create state.rs
    let mut file = File::create(format!("{}/state.rs", current_dir)).await?;
    file.write_all(state_rs::CODE.as_bytes()).await?;

    // Create state.rs
    let mut file = File::create(format!("{}/sql_types.rs", current_dir)).await?;
    file.write_all("pub use diesel::sql_types::*;".as_bytes()).await?;

    Ok(())
}