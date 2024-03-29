use structopt::StructOpt;
use support::store::models::alco_command::AlcoCommand;

use super::{add_feature::add_feature, setup::setup_project};


pub async fn web_app_main(){
    // Parse command-line arguments
    let command = AlcoCommand::from_args();
    println!("web app main: {:?}", command);
    // Perform actions based on the command
    match command {
        AlcoCommand::Setup{project_name} => {
            match setup_project(project_name).await{
                Ok(()) => println!("Rust web app setup successful!"),
                Err(e) => println!("Setup failed: {}", e),
            }
        }
        AlcoCommand::Feature{feature_name, feature_parent} => {
            match add_feature::add(feature_name.clone(), feature_parent).await{
                Ok(()) => println!("Successfully added {} feature to your web app!", &feature_name),
                Err(e) => println!("Failed adding {} feature: {}", &feature_name, e),
            }
        }
    }
}