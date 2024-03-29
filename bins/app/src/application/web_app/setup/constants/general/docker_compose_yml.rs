pub const CODE: &str = r#"version: '2'

volumes:
  postgres-data:
    driver: local

services:
  postgres:
    image: postgres:latest
    restart: always
    hostname: postgres
    container_name: postgres
    environment:
      - POSTGRES_USERNAME=postgres
      - POSTGRES_PASSWORD=pass
      - POSTGRES_DATABASE=postgres
      - POSTGRES_WAL_LEVEL=logical
    ports: 
      - "5432:5432"
"#;