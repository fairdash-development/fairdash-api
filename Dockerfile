FROM ubuntu:latest
LABEL authors="cameron"

ENTRYPOINT ["top", "-b"]

FROM rust:latest as builder
WORKDIR /app

COPY . .

# checks for railway port
ENV PORT=${PORT:-8080}
EXPOSE $PORT

RUN cargo build --release --bin fairdash-api
CMD ["./target/release/fairdash-api"]
