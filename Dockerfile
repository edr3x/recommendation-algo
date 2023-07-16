FROM rust:1.69 as planner

WORKDIR /app

RUN cargo install cargo-chef

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.69 as cacher

WORKDIR /app

RUN cargo install cargo-chef

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.69 AS builder

ENV USER=webuser
ENV UID=6969

RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  "${USER}"

COPY . /app

WORKDIR /app

COPY --from=cacher /app/target target

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

COPY --from=builder /app/target/release/recom-algo /app/recom-algo

WORKDIR /app

CMD ["./recom-algo"]
