FROM ubuntu:latest
LABEL authors="cameron"

ENTRYPOINT ["top", "-b"]

FROM ubuntu:latest

WORKDIR /app

COPY . .

RUN cargo install dioxus-cli
RUN rustup target add wasm32-unknown-unknown

RUN dx build --features web --release
CMD cargo run --features ssr --release