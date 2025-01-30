# Running the API:

- Docker Compose

  - `docker compose build`
  - `docker compose up`

- API Base URL: http://localhost:3000/api
- Database Viewer: http://localhost:8081/

- If not using Docker Compose: `watchexec --restart --watch src --exts rs -- cargo run`

  - Note this command requires the installation of `watchexec-cli`
  - Use this command to add it: `cargo install --locked watchexec-cli`

## Dependencies

cargo new axum-pg-docker-crud
cargo add axum
cargo add tokio --features=full
cargo add serde --features=derive
cargo add diesel --features=chrono,postgres
cargo add diesel-async --features bb8,postgres
