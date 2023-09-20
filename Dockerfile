FROM rust:latest as builder
WORKDIR /opt/build
COPY src src/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo build --release

FROM almalinux:minimal as app
COPY --from=builder /opt/build/target/release/cloud-build-notifier /usr/local/bin/cloud-build-notifier
ENTRYPOINT ["/usr/local/bin/cloud-build-notifier"]
