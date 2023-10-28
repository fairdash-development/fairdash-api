FROM ubuntu:latest
LABEL authors="subtosharki"

ENTRYPOINT ["top", "-b"]

FROM rust:latest as builder
WORKDIR /app
EXPOSE 3000
COPY . .
RUN cargo build --release --bin fairdash-api
CMD ["./target/release/fairdash-api"]