use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "alco", about = "Make rust life easier")]
pub enum AlcoCommand {
    #[structopt(about = "Setup up rust web app project")]
    Setup{
        #[structopt(help = "The name given to the rust project")]
        project_name: String,
    },
    
    #[structopt(about = "Add a feature to rust web app")]
    Feature{
        #[structopt(help = "The name given to the feature")]
        feature_name: String,

        #[structopt(help = "Used in case where feature is a subfeature")]
        feature_parent: Option<String>,
    },

}
