ARG BUILDER_IMAGE_NAME
ARG SCCACHE_GCS_BUCKET

FROM ${BUILDER_IMAGE_NAME} as builder

WORKDIR /echo_app

COPY src /echo_app/src
COPY Cargo.lock /echo_app
COPY Cargo.toml /echo_app

ENV SCCACHE_GCS_BUCKET ${SCCACHE_GCS_BUCKET}

RUN RUSTC_WRAPPER=$(which sccache) cargo build --release

# --------

FROM debian:10.4
COPY --from=builder /echo_app/target/release/app /usr/local/bin/echo-app
CMD ["echo-app"]
