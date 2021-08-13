FROM rust:1-alpine

WORKDIR /app

COPY . .

RUN cargo install --path .

CMD sleep infinity
