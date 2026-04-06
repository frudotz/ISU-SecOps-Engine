# Stage 1: Build the Rust application
FROM rust:1.80-slim-bookworm AS builder

# Install necessary build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Create working directory
WORKDIR /usr/src/app

# Copy the Cargo files and project source
COPY . .

# Build for release
RUN cargo build --release

# Stage 2: Create a minimal runtime environment
FROM debian:bookworm-slim

# Install necessary runtime libraries
# We include 'ca-certificates' to allow reqwest to make HTTPS API calls,
# and 'socat' to proxy the hardcoded 127.0.0.1 web server to 0.0.0.0.
RUN apt-get update && apt-get install -y ca-certificates socat && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/pentester /app/pentester

# Copy required web files and assets
COPY --from=builder /usr/src/app/index.html /app/index.html
COPY --from=builder /usr/src/app/assets /app/assets

# Create an entrypoint script to handle the localhost binding limitation.
# Since the app code specifically binds to 127.0.0.1:3000, we forward it to 0.0.0.0:8080
# allowing external access to the container when the web server is running.
RUN echo '#!/bin/sh\n\
    if [ "$1" = "web" ]; then\n\
    echo "[!] Web mode detected. Starting socat to forward 0.0.0.0:8080 to internal 127.0.0.1:3000"\n\
    socat TCP-LISTEN:8080,fork,bind=0.0.0.0 TCP:127.0.0.1:3000 &\n\
    fi\n\
    exec /app/pentester "$@"' > /app/entrypoint.sh && chmod +x /app/entrypoint.sh

# Expose the proxied port for the Web UI
EXPOSE 8080

ENTRYPOINT ["/app/entrypoint.sh"]

# Set default command so running `docker run <image>` shows help
CMD ["--help"]
