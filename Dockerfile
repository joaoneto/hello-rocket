FROM rust:1.56.1

WORKDIR /usr/src/hello-rust
COPY . .

RUN cargo install --path .

CMD ./target/release/hello-rocket
