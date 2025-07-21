FROM rust:1.88

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release
RUN rm -r src

COPY . .

RUN cargo build --release

CMD ["./target/release/quote-bot"]