pub const CODE: &str = r#"[package]
name = "infrastructure"
version = "0.2.0"
edition = "2021"
publish = false

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[features]
dev = []

[lib]
doctest = false

[dependencies]
config = { path = "../config" }
error = { path = "../error" }
serde = "*"
serde_json = "*"

r2d2 = "0.8.10"
r2d2_redis = "0.14.0"
redis="0.25.0"

diesel = { version = "2.0.0-rc.0", features = [
    "postgres",
    "chrono",
    "r2d2",
    "serde_json",
    "64-column-tables",
] }
diesel_migrations = "2.0.0-rc.0"

[dev-dependencies]
config = { path = "../config", features = ["dev"] }

"#;