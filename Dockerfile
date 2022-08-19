FROM rust:1.63-bullseye as builder

WORKDIR /usr/src/time_to_rust
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

COPY --from=builder /usr/src/time_to_rust/target/release/time_to_rust /usr/local/bin/time_to_rust
EXPOSE 8080

CMD ["time_to_rust"]