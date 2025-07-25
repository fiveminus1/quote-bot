FROM rust:1.88 AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y libpq-dev pkg-config

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
  libssl3 \
  sqlite3 \
  libsqlite3-dev \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/quote-bot .

CMD ["./quote-bot"]