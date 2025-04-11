FROM rust:1.83.0-bookworm AS build

WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=build /app/target/release/actix-app /app/server
EXPOSE 8080
CMD ["/app/server"]

# docker build -t rust-actix-app:glibc .
# docker run --rm -it -p8080:8080 rust-actix-app:glibc
