# Base image extends rust:nightly which extends debian:buster-slim
FROM rust:latest as builder

WORKDIR /usr/src/teletxt
COPY . .

RUN cargo build --release

# Copy the binary into a new container for a smaller docker image
FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl tzdata ca-certificates && update-ca-certificates


COPY --from=builder /usr/src/teletxt/target/release/teletxt /
USER root

CMD ["/teletxt"]
