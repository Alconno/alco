pub const CODE: &str = r#"[package]
edition = "2021"
name = "support"
publish = false
version = "0.2.0"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
doctest = false

[dependencies]
env_logger = "0.11.1"
log = "0.4.20"
chrono = { version = "0.4.33", features = ["serde"] }
serde = "*"
serde_json = "1.0.113"
reqwest = { version = "0.11.24" }
futures = "0.3.30"
futures-util = "0.3.21"
tokio = { version = "1.35.1", features = ["full"] }
uuid = { version = "1.4.1", features = ["v4", "serde"] }
lettre = "0.11.4"
dotenv = "0.15.0"

# Workspace
config = { path = "../config" }
error = { path = "../error" }
infrastructure = { path = "../infrastructure", features = ["dev"] }

# database
diesel = { version = "2.1.4", features = [
    "postgres",
    "chrono",
    "r2d2",
    "serde_json",
    "64-column-tables",
    "numeric",
    "uuid",
    "time",
] }
diesel_full_text_search = "2.1.1"
diesel_migrations = "2.0.0-rc.0"

# Http stuff
actix-multipart = "0.6.0"
actix-web = "4"
"#;