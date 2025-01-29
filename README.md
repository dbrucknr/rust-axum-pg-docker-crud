# Running the API:

- If not using Docker: `watchexec --restart --watch src --exts rs -- cargo run`
  - Note this command requires the installation of `watchexec-cli`
  - Use this command to add it: `cargo install --locked watchexec-cli`

cargo new axum-pg-docker-crud
cargo add axum
cargo add tokio --features=full
cargo add serde --features=derive
