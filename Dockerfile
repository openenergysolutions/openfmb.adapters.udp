FROM rust:alpine3.14 as builder

RUN apk update && apk add --no-cache \ 
    build-base \
    linux-headers \
    libressl-dev \
    libpcap-dev \
    protobuf-dev

# Build the project
COPY . /adapters

WORKDIR /adapters

RUN cargo build --release

# build running image
FROM alpine:3.14

COPY --from=builder /adapters/target/release/udp-adapter /usr/local/bin/

WORKDIR /usr/local/bin/

ENV RUST_LOG=info

ENTRYPOINT ["udp-adapter"]
