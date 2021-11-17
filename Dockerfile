FROM rust:1-alpine

RUN apk add gcc g++ mingw-w64-gcc libc-dev make cmake bash clang patch \
            libxml2-dev openssl openssl-dev fts-dev bsd-compat-headers xz git
            
WORKDIR /app

COPY . .

RUN cargo install --path .
RUN cargo install cross

RUN rustup target add x86_64-pc-windows-gnu
RUN rustup target add x86_64-apple-darwin

CMD sleep infinity
