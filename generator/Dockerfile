FROM rust:alpine
RUN apk update
RUN apk add --no-cache gcc clang lld openssl-dev musl-dev
RUN rustup target add "$(uname -m)-unknown-linux-musl"
