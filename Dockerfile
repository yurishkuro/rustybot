FROM rust:1.79

WORKDIR /app

COPY . /app

RUN cargo build --release

CMD ["target/release/rustybot"]
