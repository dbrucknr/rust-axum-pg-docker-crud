# -------------------------------------
# 1. Base image
# -------------------------------------
# Use the official Rust image to build our application
FROM rust:1.84 AS builder
# Install system dependencies (for Postgres)
RUN apt-get update && apt-get install -y libpq-dev netcat-openbsd
# Install watchexec
RUN cargo install watchexec-cli
# Install Diesel-CLI (Database Migrations)
RUN cargo install diesel_cli --no-default-features --features postgres
# Create a new empty shell project
WORKDIR /app
# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./
# Copy the source code
COPY src ./src
# Build our application
RUN cargo install --path .
# -------------------------------------
# 2. Development stage
# -------------------------------------
# Copy everything into /app
COPY . /app

# Copy the entrypoint script into the image
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Set the entrypoint
ENTRYPOINT ["/entrypoint.sh"]

# # Auto Reload Docker Container when any file changes in src
# CMD ["watchexec", "--restart", "--watch", "src", "--exts", "rs", "cargo", "run"]
# Entrypoint: Run Diesel migrations then start the server with watchexec
# CMD ["sh", "-c", "diesel migration run && watchexec --restart --watch src --exts rs cargo run"]