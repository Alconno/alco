pub const CODE: &str = r#"[package]
name = "error"
version = "0.1.0"
edition = "2021"
publish = false

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[lib]
test = false
doctest = false

[dependencies]
# Http stuff
actix-web = "4.3.1"
reqwest = { version = "0.11.10", features = ["json", "blocking"] }

# database stuff
r2d2 = "0.8.9"
diesel = { version = "2.1.0", features = ["postgres"] }

# serde
serde = "1.0"
serde_json = "^1.0.0"

#helpers
validr = "0.3.0"
thiserror = "1.0.56"
lettre = "0.11.4"

"#;