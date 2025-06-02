# swdestinydb_rust_api

A fast, modular, and observable REST API for Star Wars: Destiny cards and sets, built with [Rust](https://www.rust-lang.org/) and [Axum](https://github.com/tokio-rs/axum).

> **Note:**
> This API is for **educational purposes** and demonstrates best practices in Rust backend development, observability, modularity, and API documentation.

---

## Features

- **RESTful Endpoints**: Retrieve cards, sets, and search by query.
- **Versioned API**: All endpoints are under `/v1` for future-proofing.
- **OpenAPI Documentation**: Interactive Swagger UI at `/swagger-ui`.
- **Prometheus Metrics**: Exposes `/metrics` endpoint for monitoring.
- **Health Check**: Simple `/health` endpoint for readiness/liveness probes.
- **Centralized Error Handling**: Consistent error responses.
- **Structured Logging**: Uses `tracing` for production-ready logs.
- **Modular Architecture**: Handlers, services, models, and utilities are cleanly separated.

---

## Endpoints

| Method | Path                      | Description                        |
|--------|---------------------------|------------------------------------|
| GET    | `/v1/sets`                | List all sets                      |
| GET    | `/v1/cards/:set_code`     | List all cards in a set            |
| GET    | `/v1/card/:card_id`       | Get details for a specific card    |
| GET    | `/v1/find?q=...`          | Search for cards by query          |
| GET    | `/health`                 | Health check                       |
| GET    | `/metrics`                | Prometheus metrics                 |
| GET    | `/swagger-ui`             | Swagger UI (OpenAPI docs)          |

---

## Observability

- **Metrics**: All endpoints record request count, success/error count, and request duration, exposed in Prometheus format.
- **Logging**: All important events and errors are logged using `tracing`.
- **Health Check**: `/health` returns `200 OK` if the API is up.

---

## Project Structure

```
src/
    app_state.rs
    handlers/
        cards.rs
        mod.rs
        sets.rs
    main.rs
    models/
        card_response.rs
        mod.rs
        set_response.rs
    services/
        card_service.rs
        mod.rs
        sets_service.rs
    utils/
        metrics.rs
        mod.rs
```

---

## How to Run

1. **Install Rust**
     [https://rustup.rs/](https://rustup.rs/)

2. **Set environment variables**
     Create a `.env` file with:
     ```
     API_BASE_URL=https://your-data-source-url
     ```

3. **Run the server**
     ```sh
     cargo run
     ```

4. **Access the API**
     - Swagger UI: [http://localhost:3000/swagger-ui](http://localhost:3000/swagger-ui)
     - Metrics: [http://localhost:3000/metrics](http://localhost:3000/metrics)
     - Health: [http://localhost:3000/health](http://localhost:3000/health)

---

## Tech Stack

- Rust
- Axum
- Utoipa (OpenAPI)
- Prometheus (metrics)
- Tracing (logging)
- dotenv (env config)

---

## License

MIT

---

## About

This project is a template for robust, observable, and maintainable Rust APIs, using best practices found in production environments.
Created for educational purposes.