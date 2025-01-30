# -------------------------------------
# 1. Base image
# -------------------------------------
# Use the official Rust image to build our application
FROM rust:1.84 AS builder
# Install watchexec
RUN cargo install watchexec-cli
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
FROM base AS dev
# Copy everything into /app
COPY . /app
# Auto Reload Docker Container when any file changes in src
CMD ["watchexec", "--restart", "--watch", "src", "--exts", "rs", "cargo", "run"]