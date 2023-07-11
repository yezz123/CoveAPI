FROM rust:1-buster as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install openssl -y

# Preload Dependencies
RUN  cargo new /work/app
WORKDIR /work/app

COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
RUN cargo build --release --target x86_64-unknown-linux-musl

# Build Application
COPY ./src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM nginx:1.23.0-alpine

WORKDIR /app

# Unlink access log from stdout (to allow for analysis)
RUN rm /var/log/nginx/access.log

COPY --from=builder /work/app/target/x86_64-unknown-linux-musl/release/coveapi /
COPY ./nginx/nginx.conf /etc/nginx/nginx.conf

CMD ["/coveapi"]
