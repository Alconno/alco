pub const CODE: &str = r#"[package]
name = "rust-api"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# Workspace
config = { path = "../../libs/config" }
error = { path = "../../libs/error" }
infrastructure = { path = "../../libs/infrastructure" }
support = { path = "../../libs/support" }

# codegen stuff
async-trait = "0.1"

# Http stuff
actix-web = "4.3.0"
actix-rt = "2.9.0"
actix-service = "2.0.2"
reqwest = { version = "0.11.10", features = ["json"] }
http = "1.0.0"

# database stuff
diesel = { version = "2.1.0", features = [
  "postgres",
  "chrono",
  "r2d2",
  "serde_json",
] }

# serde
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

#helpers
dotenv = "0.15.0"
env_logger = "0.11.1"
log = "0.4.17"
validr = "0.3.1"

# docs
openapi = "0.1.5"
"#;
