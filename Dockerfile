# Build Stage
FROM rust:1-buster as builder

# set correct linker
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN yes | apt install gcc-x86-64-linux-gnu

# Create a new empty Rust project
WORKDIR /work/app
RUN USER=root cargo new --bin coveapi
WORKDIR /work/app/coveapi

# Copy dependency manifests
COPY ./Cargo.lock ./Cargo.toml ./
# Build and cache dependencies
RUN cargo build --release --target x86_64-unknown-linux-musl

# Copy source code
COPY ./src ./src
# Build the application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Final Stage
FROM nginx:1.23.0-alpine

# Remove default nginx configurations
RUN rm /var/log/nginx/access.log

# Copy the built binary from the build stage
COPY --from=builder /work/app/coveapi/target/x86_64-unknown-linux-musl/release/coveapi /usr/local/bin/coveapi

# Copy nginx configuration
COPY ./nginx/nginx.conf /etc/nginx/nginx.conf

# Set the binary as the default command to run
CMD ["coveapi"]
