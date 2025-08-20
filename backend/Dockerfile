FROM rust:1.75-slim as builder
WORKDIR /app
COPY Cargo.toml .
# Create a dummy main to cache dependencies
RUN mkdir src && printf "fn main(){}" > src/main.rs
RUN cargo build --release || true
# Now copy real source
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
ENV PORT=3001
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
EXPOSE 3001
CMD ["backend"]
