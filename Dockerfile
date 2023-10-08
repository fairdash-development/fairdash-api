FROM ubuntu:latest
LABEL authors="subtosharki"

ENTRYPOINT ["top", "-b"]

FROM rust:latest as builder
WORKDIR /app
EXPOSE 3000
COPY . .
ENV DATABASE_URL=postgresql://postgres:9gkfh1o6l8fTE2Qoox1U@containers-us-west-37.railway.app:7189/railway
RUN cargo build --release --bin fairdash-api
CMD ["./target/release/fairdash-api"]