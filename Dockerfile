FROM rust:alpine3.10 as builder
RUN USER=root cargo init --name betting
COPY Cargo.toml .
RUN cargo build --release
COPY src src
RUN cargo build --release
CMD ["app"]

FROM alpine:3.10
COPY --from=builder /target/release/betting /bin/
CMD betting