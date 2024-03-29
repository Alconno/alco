use std::path::Path;

use error::Error;
use support::helpers::find_directory::find_directory;

use super::service::add_feature_code::add_feature_code;

/// Functionality of Feature command
pub async fn add(feature_name: String, feature_parent: Option<String>) -> Result<(), Error> {
    let mut application_dir = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("bins/app/src/application");

    if let Some(feature_parent) = feature_parent{
        application_dir = find_directory(&application_dir, &feature_parent)
            .expect("Could not find the feature_parent directory");
    }  

    if Path::new(&application_dir).exists() {
        add_feature_code(
            feature_name,
            application_dir.to_string_lossy().to_string(),
        ).await?;
    } else {
        return Err(Error::NotFoundWithCause(
            "Failed to find application folder. Make sure you're in project root directory!".to_string()
        ));
    }

    Ok(())
}