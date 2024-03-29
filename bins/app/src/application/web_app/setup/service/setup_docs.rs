use error::Error;
use tokio::{fs::{self, File}, io::AsyncWriteExt};

use crate::application::web_app::setup::constants::docs::*;

pub async fn setup_docs(project_name: String) -> Result<(), Error>{
    let mut current_dir = project_name;
    
    // Enter docs dir
    current_dir.push_str("/docs");

    // Create tsconfig.json
    let mut file = File::create(format!("{}/tsconfig.json", &current_dir)).await?;
    file.write_all(tsconfig_json::CODE.as_bytes()).await?;

    // Create package.json
    let mut file = File::create(format!("{}/package.json", &current_dir)).await?;
    file.write_all(package_json::CODE.as_bytes()).await?;

    // Create index.ts
    let mut file = File::create(format!("{}/index.ts", &current_dir)).await?;
    file.write_all(index_ts::CODE.as_bytes()).await?;



    // Save for later use at component generation
    let saved_current_dir = current_dir.clone();
   


    // Paths
    current_dir.push_str("/paths");
    fs::create_dir(&current_dir).await?;

    // .gitkeep
    File::create(format!("{}/.gitkeep", &current_dir)).await?;

    // private ( app feature routes )
    fs::create_dir(format!("{}/private", &current_dir)).await?;

    // public ( routes defined in configure )
    current_dir.push_str("/public");
    fs::create_dir(&current_dir).await?;

    // Create HealthCheck.ts
    let mut file = File::create(format!("{}/HealthCheck.ts", &current_dir)).await?;
    file.write_all(paths::health_check_ts::CODE.as_bytes()).await?;



    // Components
    current_dir = saved_current_dir;

    // base_url.rs
    current_dir.push_str("/components");
    fs::create_dir(&current_dir).await?;

    fs::create_dir(&format!("{}/callbacks", &current_dir)).await?;
    File::create(format!("{}/callbacks/.gitkeep", &current_dir)).await?;

    fs::create_dir(&format!("{}/examples", &current_dir)).await?;
    File::create(format!("{}/examples/.gitkeep", &current_dir)).await?;

    fs::create_dir(&format!("{}/headers", &current_dir)).await?;
    File::create(format!("{}/headers/.gitkeep", &current_dir)).await?;

    fs::create_dir(&format!("{}/links", &current_dir)).await?;
    File::create(format!("{}/links/.gitkeep", &current_dir)).await?;

    fs::create_dir(&format!("{}/parameters", &current_dir)).await?;
    File::create(format!("{}/parameters/.gitkeep", &current_dir)).await?;

    fs::create_dir(&format!("{}/request-bodies", &current_dir)).await?;
    File::create(format!("{}/request-bodies/.gitkeep", &current_dir)).await?;
    
    fs::create_dir(&format!("{}/schemas", &current_dir)).await?;
    File::create(format!("{}/schemas/.gitkeep", &current_dir)).await?;

    fs::create_dir(&format!("{}/security-schemas", &current_dir)).await?;
    File::create(format!("{}/security-schemas/.gitkeep", &current_dir)).await?;



    // Responses
    current_dir.push_str("/responses");
    fs::create_dir(&current_dir).await?;

    // Create BadRequestError.ts
    let mut file = File::create(format!("{}/BadRequestError.ts", &current_dir)).await?;
    file.write_all(components::responses::BAD_REQUEST.as_bytes()).await?;

    // Create DieselError.ts
    let mut file = File::create(format!("{}/DieselError.ts", &current_dir)).await?;
    file.write_all(components::responses::DIESEL.as_bytes()).await?;

    // Create ForbiddenError.ts
    let mut file = File::create(format!("{}/ForbiddenError.ts", &current_dir)).await?;
    file.write_all(components::responses::FORBIDDEN.as_bytes()).await?;

    // Create InternalServerError.ts
    let mut file = File::create(format!("{}/InternalServerError.ts", &current_dir)).await?;
    file.write_all(components::responses::INTERNAL_SERVER.as_bytes()).await?;

    // Create NotFoundError.ts
    let mut file = File::create(format!("{}/NotFoundError.ts", &current_dir)).await?;
    file.write_all(components::responses::NOT_FOUND.as_bytes()).await?;

    // Create PayloadTooLargeError.ts
    let mut file = File::create(format!("{}/PayloadTooLargeError.ts", &current_dir)).await?;
    file.write_all(components::responses::PAYLOAD_TOO_LARGE.as_bytes()).await?;

    // Create UnauthorizedError.ts
    let mut file = File::create(format!("{}/UnauthorizedError.ts", &current_dir)).await?;
    file.write_all(components::responses::UNAUTHORIZED.as_bytes()).await?;

    // Create UnprocessableEntityError.ts
    let mut file = File::create(format!("{}/UnprocessableEntityError.ts", &current_dir)).await?;
    file.write_all(components::responses::UNPROCESSABLE_ENTITY.as_bytes()).await?;



    Ok(())
}