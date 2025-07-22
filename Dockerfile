FROM rust:1.88 AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y libsqlite3-dev pkg-config

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release
RUN rm -r src

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
  libssl3 \
  sqlite3 \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/quote-bot .

CMD ["./quote-bot"]