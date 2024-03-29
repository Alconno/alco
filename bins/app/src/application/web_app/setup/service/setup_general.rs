use error::Error;
use tokio::{fs::{self, File}, io::AsyncWriteExt};

use crate::application::web_app::setup::constants::general::*;

pub async fn setup_general(project_name: String) -> Result<bool, Error>{

    // Create libs folder
    fs::create_dir(format!("{}/libs", &project_name)).await?;

    // Create docs folder
    fs::create_dir(format!("{}/docs", &project_name)).await?;

    // Global cargo.toml
    let mut file = File::create(format!("{}/Cargo.toml", project_name)).await?;
    file.write_all(cargo_toml::CODE.as_bytes()).await?;

    // Create Package.json
    let mut file = File::create(format!("{}/Package.json", project_name)).await?;
    file.write_all(package_json::CODE.as_bytes()).await?;

    // .env folder
    let mut file = File::create(format!("{}/.env", project_name)).await?;
    file.write_all(env::CODE.as_bytes()).await?;

    // .env.example folder (  will be same as .env  )
    let mut file = File::create(format!("{}/.env.example", project_name)).await?;
    file.write_all(env::CODE.as_bytes()).await?;

    // docker-compose.yml
    let mut file = File::create(format!("{}/docker-compose.yml", project_name)).await?;
    file.write_all(docker_compose_yml::CODE.as_bytes()).await?;

    // README.md
    let mut file = File::create(format!("{}/README.md", project_name)).await?;
    file.write_all("Your README.".as_bytes()).await?;

    // .gitignore
    let mut file = File::create(format!("{}/.gitignore", project_name)).await?;
    file.write_all(gitignore::CODE.as_bytes()).await?;

    Ok(true)
}