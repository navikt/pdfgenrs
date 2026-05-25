FROM clux/muslrust:stable AS builder
RUN cargo --version && cargo install cargo-auditable
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY fonts ./fonts
ENV RUSTFLAGS='-C target-feature=+crt-static'
RUN cargo auditable build --release

FROM gcr.io/distroless/static-debian13:nonroot
WORKDIR /app
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/pdfgenrs /app/pdfgenrs
COPY --from=builder /build/fonts /app/fonts

EXPOSE 8080
CMD ["/app/pdfgenrs"]
