pub const CODE: &str = r#"{"name": "docs",
    "version": "1.0.0",
    "description": "",
    "main": "index.ts",
    "author": "Alconno",
    "license": "ISC",
    "scripts": {
        "generate": "openapi-ts-builder --format=yaml ./docs/index.ts ./openapi.yaml && openapi-ts-builder --format=json ./docs/index.ts ./openapi.json"
    },
    "dependencies": {
        "express": "^4.18.3",
        "openapi-ts-builder": "^1.0.3",
        "typescript": "^4.6.2"
    },
    "devDependencies": {
        "@types/node": "^17.0.8"
    }
}"#;
