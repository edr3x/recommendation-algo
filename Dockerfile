FROM rust:1.69 AS builder

COPY . .

RUN cargo build --release

FROM debian:stable

RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*

COPY --from=builder ./target/release/recom-algo ./recom-algo

CMD ["/recom-algo"]
