FROM rust:slim AS builder

ARG TARGETARCH

RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*
RUN if [ "$TARGETARCH" = "arm64" ]; then \
      rustup target add aarch64-unknown-linux-musl; \
    else \
      rustup target add x86_64-unknown-linux-musl; \
    fi
RUN cargo install cargo-auditable

WORKDIR /build
ENV RUSTFLAGS='-C target-feature=+crt-static'
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY fonts ./fonts

RUN if [ "$TARGETARCH" = "arm64" ]; then \
      cargo auditable build --release --target aarch64-unknown-linux-musl && \
      cp target/aarch64-unknown-linux-musl/release/pdfgenrs /pdfgenrs; \
    else \
      cargo auditable build --release --target x86_64-unknown-linux-musl && \
      cp target/x86_64-unknown-linux-musl/release/pdfgenrs /pdfgenrs; \
    fi

FROM gcr.io/distroless/static-debian13:nonroot
WORKDIR /app
COPY --from=builder /pdfgenrs /app/pdfgenrs
COPY --from=builder /build/fonts /app/fonts

EXPOSE 8080
CMD ["/app/pdfgenrs"]
