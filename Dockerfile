# Use the official Rust image as a base
FROM rust:1.70 as builder

# Create a new directory for the app
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Create a new stage with a minimal image
FROM debian:bullseye-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the built binary from builder stage
COPY --from=builder /usr/src/app/target/release/rust_log_server /usr/local/bin/

# Create logs directory
RUN mkdir -p /app/logs

# Set working directory
WORKDIR /app

# Expose the port the app runs on
EXPOSE 8080

# Command to run the binary
CMD ["rust_log_server"]
