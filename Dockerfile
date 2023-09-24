FROM ubuntu:latest
LABEL authors="subtosharki"

ENTRYPOINT ["top", "-b"]

FROM rust:latest as builder
WORKDIR /app
EXPOSE 8000
COPY . .
ENV DATABASE_URL=mongodb://mongo:PmcOXWxm5pXlHPD2b3Vy@containers-us-west-84.railway.app:5445/
RUN cargo build --release --bin fairdash-api
CMD ["./target/release/fairdash-api"]