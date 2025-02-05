# Builder stage
FROM rust:bookworm as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy only Cargo.toml and Cargo.lock first to cache dependencies
COPY Cargo.* ./

# Copy the real source code
COPY src src/

# Build the actual application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install only runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copy the binary with the correct name
COPY --from=builder /app/target/release/fishing-harbor ./fishing-harbor

CMD ["fishing-harbor"]