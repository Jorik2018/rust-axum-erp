# Use a Rust base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/bin

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Copy the rest of the application source code
COPY . .

# Build the application
RUN cargo build --release

# Create a new stage to keep the final image lightweight
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /usr/src/bin

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/bin/target/release/bin .

# Expose any necessary ports
EXPOSE 8080

# Command to run the application
CMD ["./bin"]
