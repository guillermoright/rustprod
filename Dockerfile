# Stage 1: Build the application
FROM rust:1.81-slim-bullseye AS builder

# Install required system dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && apt-get clean

# Set the working directory
WORKDIR /app

# Copy the entire project (including Cargo files and src) directly
COPY . .

# Build the project in release mode
RUN cargo build --release --locked

# Stage 2: Create a minimal runtime image
FROM debian:bullseye-slim

# Install runtime dependencies for OpenSSL
RUN apt-get update && apt-get install -y \
    libssl1.1 ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/zero2prod /app/zero2prod
# Copy the configuration.yml file into the runtime image
COPY --from=builder /app/configurationprod.yml /app/configuration.yml 

# Expose any necessary ports
EXPOSE 8080

# Run the compiled binary
ENTRYPOINT ["./zero2prod"]
