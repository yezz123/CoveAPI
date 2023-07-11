FROM rust:1-buster as builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
COPY ./src ./src

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM nginx:1.23.0-alpine

WORKDIR /app

# Unlink access log from stdout (to allow for analysis)
RUN rm /var/log/nginx/access.log

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/coveapi /
COPY ./nginx/nginx.conf /etc/nginx/nginx.conf
COPY ./dump/coveapi.toml ./coveapi.toml
COPY ./dump/swagger.json ./dump/swagger.json

CMD ["/coveapi"]
