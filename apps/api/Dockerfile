# Start with a rust image that includes cargo-chef
FROM rust:1-slim-bookworm AS chef
USER root
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install cargo-chef
WORKDIR /app

# Planner stage: Compute the recipe file
FROM chef AS planner
COPY . .
# Remove Tauri from workspace to avoid installing GUI dependencies
RUN sed -i 's/"apps\/client-web\/src-tauri"//g' Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage: Build dependencies and then the application
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this will be cached if recipe.json hasn't changed
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release -p lean-wms-api

# Runtime stage: Minimal image
FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Install OpenSSL (required for many Rust networking crates) and ca-certificates
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -r appuser && useradd -r -g appuser appuser

# Copy the binary from the builder stage
COPY --from=builder --chown=appuser:appuser /app/target/release/lean-wms-api /app/

# Switch to non-root user
USER appuser

# Expose the API port
EXPOSE 3000

# Set the entrypoint
ENTRYPOINT ["./lean-wms-api"]
