FROM rust:latest AS builder
RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-auditable

WORKDIR /build
ENV RUSTFLAGS='-C target-feature=+crt-static'
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY fonts ./fonts

ARG TARGETARCH
RUN RUST_TARGET="$([ "$TARGETARCH" = arm64 ] && echo aarch64 || echo x86_64)-unknown-linux-musl" \
    && rustup target add "$RUST_TARGET" \
    && cargo auditable build --release --target "$RUST_TARGET" \
    && cp "target/$RUST_TARGET/release/pdfgenrs" /pdfgenrs

FROM gcr.io/distroless/static-debian13:nonroot
WORKDIR /app
COPY --from=builder /pdfgenrs /app/pdfgenrs
COPY --from=builder /build/fonts /app/fonts

EXPOSE 8080
CMD ["/app/pdfgenrs"]
