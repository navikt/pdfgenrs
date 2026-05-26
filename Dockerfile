FROM clux/muslrust:stable AS builder
RUN cargo install cargo-auditable
WORKDIR /build
ENV RUSTFLAGS='-C target-feature=+crt-static'
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs && \
    cargo auditable build --release && \
    rm -f target/x86_64-unknown-linux-musl/release/deps/pdfgenrs*
COPY src ./src
COPY fonts ./fonts
RUN cargo auditable build --release

FROM gcr.io/distroless/static-debian13:nonroot
WORKDIR /app
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/pdfgenrs /app/pdfgenrs
COPY --from=builder /build/fonts /app/fonts

EXPOSE 8080
CMD ["/app/pdfgenrs"]
