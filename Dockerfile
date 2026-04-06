# 1. stage → build
FROM rust:1.76 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

# 2. stage → minimal runtime
FROM debian:bookworm-slim

WORKDIR /app

# sadece binary kopyalanır
COPY --from=builder /app/target/release/pentester /usr/local/bin/pentester

CMD ["pentester"]
