pub const CODE: &str = r#"[package]
name = "config"
version = "0.2.0"
edition = "2021"
publish = false

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
doctest = false

[dependencies]
error = { path = "../error" }
dotenv = "*"
serde = "*"
serde_json = "*"
reqwest = { version = "0.11.10", features = ["json", "blocking"] }

[features]
dev = []

"#;