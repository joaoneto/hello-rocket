FROM rust:1.56.1 as builder

WORKDIR /usr/src/hello-rocket
COPY . .
RUN cargo install --path .

FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/hello-rocket /usr/local/bin/hello-rocket
CMD hello-rocket