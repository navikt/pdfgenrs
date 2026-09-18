FROM clux/muslrust:stable@sha256:bde72f6203d191462cb6bb3f24a41b1bd8edf90788962b11715d4df54144894c AS builder
RUN cargo install cargo-auditable --version 0.7.5
WORKDIR /build
ENV RUSTFLAGS='-C target-feature=+crt-static'
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY fonts ./fonts
RUN cargo auditable build --release --locked

FROM gcr.io/distroless/static-debian13:nonroot@sha256:e2e927ec666bae08560abb3c55d0659eceabb657f56b6782ab500a9fc7f555e3
WORKDIR /app
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/pdfgenrs /app/pdfgenrs
COPY --from=builder /build/fonts /app/fonts

EXPOSE 8080
CMD ["/app/pdfgenrs"]
