FROM rust:alpine
RUN apk update
RUN apk add --no-cache gcc clang lld openssl-dev musl-dev
RUN rustup target add x86_64-unknown-linux-musl
