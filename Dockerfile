FROM ekidd/rust-musl-builder:nightly

USER rust

RUN cargo install cargo-watch
RUN cargo install cargo-add

RUN mkdir src
RUN touch src/lib.rs
ADD Cargo.lock Cargo.toml ./

RUN cargo build
RUN cargo build --tests

ENV ROCKET_ADDRESS 0.0.0.0
