FROM ubuntu:latest
LABEL authors="cameron"

ENTRYPOINT ["top", "-b"]

FROM rust:latest as builder
WORKDIR /app

COPY . .

# checks for railway port
ENV PORT=${PORT:-8080}
EXPOSE $PORT

RUN cargo build --release --bin fairdash-backend
CMD ./target/release/fairdash-backend
