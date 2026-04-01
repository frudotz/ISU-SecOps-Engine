# Justfile

#set shell := ["bash", "-cu"]

set shell := ["powershell.exe", "-Command"]

# Default
default:
    just --list

# Build
build:
    cargo build

# Run CLI example
run:
    cargo run -- headers https://example.com

# Run Web
web:
    cargo run -- web

# Test
test:
    cargo test --all

# Lint
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Format
fmt:
    cargo fmt

# Full CI locally
ci:
    just fmt
    just lint
    just test