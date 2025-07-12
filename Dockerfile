FROM rust:latest

WORKDIR /usr/src/terzi

COPY . .

RUN cargo build --release

CMD [ "/usr/src/terzi/target/release/terzi" ] 