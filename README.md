# pg-server

[![v](https://img.shields.io/badge/v-0.1.0-blueviolet)]()

### Overview
Simple PostgreSQL server with simple interaction

### API:
 - create-account
 - login
 - get-details

Run with

```bash
    cargo run
    # or
    ./target/debug/pg-server
    # or
    ./target/release/pg-server
```

Test with curl:
```bash
    curl --location -X POST 'localhost:8080/create-account' \
    --header 'Content-Type: application/json' \
    --header 'Content-Type: text/plain' \
    --data-raw '{
        "email": "obi.wan.kenobi@gmail.com",
        "car": "Speeder 1625",
        "bank": "Naboo National Bank Branch, account <classified>",
    }'
```
