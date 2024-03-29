use error::Error;
use tokio::{fs::{self, File}, io::AsyncWriteExt};

use crate::application::web_app::setup::constants::libs::support::*;

pub async fn setup_libs_support(project_name: String) -> Result<(), Error>{
    let mut current_dir = project_name;
    // Enter libs dir
    current_dir.push_str("/libs");

    current_dir.push_str("/support");
    fs::create_dir(&current_dir).await?;

    // Create Cargo toml
    let mut file = File::create(format!("{}/Cargo.toml", current_dir)).await?;
    file.write_all(cargo_toml::CODE.as_bytes()).await?;

    // src folder
    current_dir.push_str("/src");
    fs::create_dir(&current_dir).await?;

    // lib folder
    let mut file = File::create(format!("{}/lib.rs", current_dir)).await?;
    file.write_all(lib_rs::CODE.as_bytes()).await?;

    // Save for later use at store generation
    let mut saved_current_dir = current_dir.clone();
   
    
    // Helpers
    current_dir.push_str("/helpers");
    fs::create_dir(&current_dir).await?;

    // mod.rs
    let mut file = File::create(format!("{}/mod.rs", current_dir)).await?;
    file.write_all("pub mod base_url;\npub mod http;".as_bytes()).await?;

    // base_url.rs
    let mut file = File::create(format!("{}/base_url.rs", current_dir)).await?;
    file.write_all(base_url_rs::CODE.as_bytes()).await?;

    // base_url.rs
    let mut file = File::create(format!("{}/http.rs", current_dir)).await?;
    file.write_all(http_rs::CODE.as_bytes()).await?;

    
    
    
    // Store ( models )
    saved_current_dir.push_str("/store");
    fs::create_dir(&saved_current_dir).await?;

    // mod.rs
    let mut file = File::create(format!("{}/mod.rs", saved_current_dir)).await?;
    file.write_all("pub mod models;".as_bytes()).await?;

    saved_current_dir.push_str("/models");
    fs::create_dir(&saved_current_dir).await?;

    // models/mod.rs
    File::create(format!("{}/mod.rs", saved_current_dir)).await?;

    Ok(())
}