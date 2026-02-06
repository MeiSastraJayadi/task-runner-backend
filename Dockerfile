# ========================
# Build stage
# ========================
FROM rust:1.90 as builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Build real app
COPY . .
RUN cargo build --release

# ========================
# Runtime stage
# ========================
FROM debian:bookworm-slim

# Install runtime deps
RUN apt-get update && apt-get install -y \
    ca-certificates \
    openssl \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/task-runner-backend /usr/local/bin/app

# Expose port
EXPOSE 8080

# Run app
CMD ["app"]
