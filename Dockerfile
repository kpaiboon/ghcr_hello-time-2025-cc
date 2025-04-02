FROM rust:1.76-slim-bookworm AS builder
WORKDIR /app

# Update Rust and Cargo
RUN rustup update

# Copy project files
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies
RUN cargo fetch

# Copy source code and build the application
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/rusty_webserver .
EXPOSE 6000
CMD ["./rusty_webserver"]
