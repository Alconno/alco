use std::io::SeekFrom;

use error::Error;
use regex::Regex;
use tokio::{fs, io::{AsyncSeekExt, AsyncWriteExt}};


pub async fn add_feature_to_configure(current_dir: &str, feature_name: &str, include_path: &str) -> Result<(), Error> {
    // Define the regex pattern to match the configure function
    let re = Regex::new(r#"pub fn configure[\s\S]*?\);"#)?;

    // Read the content of configure.rs
    let mut content = fs::read_to_string(format!("{}/configure.rs", current_dir)).await?;

    // Search for the pattern in the text
    if let Some(mat) = re.find(&content) {
        // Get the index of the end of the match
        let end_index = mat.end();

        // Insert the new content at the end of the match
        let new_content = format!("\n    add_{}_routes(pg_pool.clone(), cfg);\n", feature_name);
        
        if !content.contains(&new_content) {
            content.insert_str(end_index, &new_content);
        }

        // Open the file in write mode with Truncate option to clear its contents
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true) // Truncate option to clear the file
            .open(format!("{}/configure.rs", current_dir))
            .await?;

        // Seek to the start of the file
        file.seek(SeekFrom::Start(0)).await?;

        // Write the modified content back to the file
        file.write_all(content.as_bytes()).await?;

        // Implement the add_{}_routes function at the end of the file
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(format!("{}/configure.rs", current_dir))
            .await?;

            let add_route_function = "\nfn add_{0}_routes(p: Arc<Pg>, c: &mut web::ServiceConfig) {
    crate::application::{1}::setup::routes(p.clone(), c);
}".replace("{0}", feature_name).replace("{1}", include_path);
        
        if !content.contains(&add_route_function){
            file.write_all(add_route_function.as_bytes()).await?;
        }
    } else {
        println!("Invalid configure.rs");
    }

    Ok(())
}