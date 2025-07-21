FROM rust:1.88 as builder

WORKDIR /app

RUN apt-get update && apt-get install -y libsqlite3-dev pkg-config

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release
RUN rm -r src

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
  libssl1.1 \
  sqlite3 \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/quote-bot .

COPY user_map.json ./

CMD ["./quote-bot"]