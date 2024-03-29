

use error::Error;
use support::helpers::text_case_converters::snake_to_pascal_case;
use tokio::{fs::{self, File}, io::AsyncWriteExt};

use super::{super::constants::*, update_app_mod::add_feature_to_application_mod, update_configure::add_feature_to_configure};

/// Function to completely add a feature template to an existing rust web app project ( from root dir )
pub async fn add_feature_code(feature_name: String, mut current_dir: String) -> Result<(), Error>{

    // Create feature folder
    current_dir.push_str(&format!("/{}", feature_name));
    fs::create_dir(&current_dir).await?;

    // Define an include_path so feature can be accessed from configure file
    let relative_path = current_dir.split("/application/").nth(1).unwrap_or("");
    let include_path = relative_path.replace('/', "::");
    
    // Define an actual application_dir since current_dir could be a subfeature
    let application_dir = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("bins/app/src/application").to_string_lossy().to_string();

    // Include the new feature into application module
    add_feature_to_application_mod(&current_dir, &feature_name).await?;

    // Add feature into configure
    add_feature_to_configure(&application_dir, &feature_name, &include_path).await?;


    // mod.rs
    let mut file = File::create(format!("{}/mod.rs", current_dir)).await?;
    file.write_all("pub mod http;\npub mod contract;\npub mod data;\npub mod domain;\npub mod infrastructure;\npub mod setup;".as_bytes()).await?;

    let pascal_feature_name = snake_to_pascal_case(&feature_name);

    // contract.rs
    let mut file = File::create(format!("{}/contract.rs", current_dir)).await?;
    file.write_all(contract_rs::get_contract_code(&pascal_feature_name).as_bytes()).await?;

    // data.rs
    let mut file = File::create(format!("{}/data.rs", current_dir)).await?;
    file.write_all(data_rs::get_data_code(&pascal_feature_name).as_bytes()).await?;
    
    // domain.rs
    let mut file = File::create(format!("{}/domain.rs", current_dir)).await?;
    file.write_all(domain_rs::get_domain_code(&pascal_feature_name).as_bytes()).await?;
 
    // infrastructure.rs
    let mut file = File::create(format!("{}/infrastructure.rs", current_dir)).await?;
    file.write_all(infrastructure_rs::INFRASTRUCTURE.as_bytes()).await?;
  
    // setup.rs
    let mut file = File::create(format!("{}/setup.rs", current_dir)).await?;
    file.write_all(setup_rs::get_setup_code(
        &pascal_feature_name,
        &feature_name,
    ).as_bytes()).await?;


    // Create http folder
    current_dir.push_str("/http");
    fs::create_dir(&current_dir).await?;

    // http/mod.rs
    let mut file = File::create(format!("{}/mod.rs", current_dir)).await?;
    file.write_all(format!("pub mod handle_{0};\npub use handle_{0}::*;", feature_name).as_bytes()).await?;

    // handle.rs
    let mut file = File::create(format!("{}/handle_{}.rs", current_dir, feature_name)).await?;
    file.write_all(handle_rs::get_handle_code(
        &pascal_feature_name,
        &feature_name,
    ).as_bytes()).await?;


    Ok(())
}