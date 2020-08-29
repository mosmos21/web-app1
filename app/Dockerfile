FROM rust:1.43 as builder

WORKDIR /echo_app
COPY src /echo_app/src
COPY Cargo.lock /echo_app
COPY Cargo.toml /echo_app
RUN cargo build --release

# --------

FROM debian:10.4
COPY --from=builder /echo_app/target/release/app /usr/local/bin/echo-app
CMD ["echo-app"]
