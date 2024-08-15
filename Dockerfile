# TODO use staged builds to avoid including Rust toolchain

FROM rust:1.80

WORKDIR /app

COPY . /app

RUN cargo build --release

ENTRYPOINT ["/app/target/release/rustybot"]
