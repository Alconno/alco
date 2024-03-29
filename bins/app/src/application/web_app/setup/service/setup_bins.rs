use error::Error;
use tokio::{fs::{self, File}, io::AsyncWriteExt};

use crate::application::web_app::setup::constants::bins::*;

pub async fn setup_bins(project_name: String) -> Result<bool, Error>{
    let mut current_dir = project_name;

    // Create folders
    current_dir.push_str("/bins");
    fs::create_dir(&current_dir).await?;
    current_dir.push_str("/app");
    fs::create_dir(&current_dir).await?;

    // Create bins app Cargo toml
    let mut file = File::create(format!("{}/Cargo.toml", current_dir)).await?;
    file.write_all(cargo_toml::CODE.as_bytes()).await?;

    // Create src folder
    current_dir.push_str("/src");
    fs::create_dir(&current_dir).await?;

    // Create main.rs
    let mut file = File::create(format!("{}/main.rs", current_dir)).await?;
    file.write_all(main_rs::CODE.as_bytes()).await?;

    // Create web.rs
    let mut file = File::create(format!("{}/web.rs", current_dir)).await?;
    file.write_all(web_rs::CODE.as_bytes()).await?;

    // Create application folder
    current_dir.push_str("/application");
    fs::create_dir(&current_dir).await?;

    // application configure.rs
    let mut file = File::create(format!("{}/configure.rs", current_dir)).await?;
    file.write_all(configure_rs::CODE.as_bytes()).await?;

    // application mod.rs
    let mut file = File::create(format!("{}/mod.rs", current_dir)).await?;
    file.write_all("pub mod configure;\npub use configure::configure;".as_bytes()).await?;




    Ok(true)
}