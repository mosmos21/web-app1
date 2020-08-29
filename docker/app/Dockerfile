FROM rust:1.43 as builder

WORKDIR /echo
COPY app/ /echo
RUN cargo build --release


FROM debian:10.4
COPY --from=builder /echo/target/release/app /usr/local/bin/echo-app
CMD ["echo-app"]
