use error::Error;
use tokio::fs;

use super::service::{
    setup_bins::setup_bins, 
    setup_docs::setup_docs, 
    setup_general::setup_general, 
    setup_libs::{setup_libs_config, setup_libs_error, setup_libs_infrastructure, setup_libs_support}
};

/// Functionality of Setup command
pub async fn setup_project(project_name: String) -> Result<(), Error>{
    // Create project directory
    fs::create_dir(&project_name).await?;

    // Setup general
    setup_general(project_name.clone()).await?;
    
    // Setup bins app
    setup_bins(project_name.clone()).await?;
    
    // Setup libs
    setup_libs_config(project_name.clone()).await?;
    setup_libs_error(project_name.clone()).await?;
    setup_libs_infrastructure(project_name.clone()).await?;
    setup_libs_support(project_name.clone()).await?;

    // Setup docs
    setup_docs(project_name.clone()).await?;

    std::process::Command::new("npm")
            .arg("i")
            .current_dir(&project_name)
            .output()
            .expect("Failed to run npm i");

    Ok(())
}