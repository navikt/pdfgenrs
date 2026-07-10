FROM rust:latest AS builder
RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-auditable
WORKDIR /build
ENV RUSTFLAGS='-C target-feature=+crt-static'
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY fonts ./fonts

ARG TARGETARCH
RUN case "$TARGETARCH" in \
      arm64) TARGET=aarch64-unknown-linux-musl ;; \
      *)     TARGET=x86_64-unknown-linux-musl ;; \
    esac && \
    rustup target add "$TARGET" && \
    cargo auditable build --release --target "$TARGET" && \
    cp "target/$TARGET/release/pdfgenrs" /build/pdfgenrs

FROM gcr.io/distroless/static-debian13:nonroot
WORKDIR /app
COPY --from=builder /build/pdfgenrs /app/pdfgenrs
COPY --from=builder /build/fonts /app/fonts

EXPOSE 8080
CMD ["/app/pdfgenrs"]
