FROM clux/muslrust:stable AS builder
RUN cargo install cargo-auditable
WORKDIR /build
ENV RUSTFLAGS='-C target-feature=+crt-static'
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY fonts ./fonts
RUN cargo auditable build --release \
    && cp target/${DOCKER_TARGET_ARCH}-unknown-linux-musl/release/pdfgenrs /pdfgenrs

FROM gcr.io/distroless/static-debian13:nonroot
WORKDIR /app
COPY --from=builder /pdfgenrs /app/pdfgenrs
COPY --from=builder /build/fonts /app/fonts

EXPOSE 8080
CMD ["/app/pdfgenrs"]
