use std::path::PathBuf;

use error::Error;
use tokio::{fs::{self, OpenOptions}, io::AsyncWriteExt};


pub async fn add_feature_to_application_mod(current_dir: &str, include_path: &str) -> Result<(), Error> {
    let current_dir_path = PathBuf::from(current_dir);
    let parent_dir = current_dir_path.parent().expect("Failed to get parent directory");

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(parent_dir.join("mod.rs"))
        .await?;

    let mod_content = fs::read_to_string(parent_dir.join("mod.rs")).await?;
    if !mod_content.contains(&format!("pub mod {0};", &include_path)) {
        file.write_all(&format!("\npub mod {0};", include_path).as_bytes()).await?;
    }

    Ok(())
}