pub const CODE: &str = r#"#################################
# Database configurations
#################################
DATABASE_URL=postgres://postgres:postgres@localhost:3000/postgres
PG_DB_URL=postgres://postgres:postgres@localhost:3000/postgres
PG_POOL_MAX_SIZE=10


#################################
# Dev configurations
#################################
IS_DEV=true
IS_TEST=false
RUST_LOG=testlog

#################################
# SERVER configurations
#################################
LISTEN_ADDRESS=1.0.0.1
LISTEN_PORT=2134
"#;