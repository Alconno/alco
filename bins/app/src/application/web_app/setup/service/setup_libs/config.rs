use error::Error;
use tokio::{fs::{self, File}, io::AsyncWriteExt};

use crate::application::web_app::setup::constants::libs::config::*;

pub async fn setup_libs_config(project_name: String) -> Result<(), Error>{
    let mut current_dir = project_name;

    // Enter libs dir
    current_dir.push_str("/libs");

    current_dir.push_str("/config");
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


    Ok(())
}