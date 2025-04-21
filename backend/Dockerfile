# ----------- Build Stage -----------
FROM rust:slim as builder
WORKDIR /app

# Install required tools for SQLx (for offline builds)
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Ensure latest stable Rust (fix Cargo.lock version issue)
RUN rustup update stable

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && echo "pub fn lib() {}" > src/lib.rs
RUN cargo build --release && rm -rf src

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# ----------- Runtime Stage -----------
FROM debian:bullseye-slim
WORKDIR /app

# Install required libraries for SQLx/Postgres
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/personal-github-dashboard /app/backend

# Copy migration scripts if needed
COPY --from=builder /app/migrations /app/migrations

# Set environment variables (can be overridden)
ENV RUST_LOG=info

# Expose the app port
EXPOSE 8080

# Run the backend binary
CMD ["/app/backend"]
