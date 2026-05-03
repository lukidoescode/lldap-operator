FROM rust:1.85-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin lldap-operator

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /app/target/release/lldap-operator /lldap-operator
USER nonroot:nonroot
ENTRYPOINT ["/lldap-operator"]
