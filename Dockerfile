FROM ubuntu:latest
LABEL authors="subtosharki"

ENTRYPOINT ["top", "-b"]

FROM rust:latest as builder
WORKDIR /app
# checks for railway port
ENV PORT=${PORT:-8080}
EXPOSE $PORT
COPY . .
RUN cargo build --release --bin fairdash-api
CMD ["./target/release/fairdash-api"]