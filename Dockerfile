FROM clux/muslrust:stable as builder

WORKDIR /build
COPY . .
ENV RUSTFLAGS='-C target-feature=+crt-static'

RUN cargo build --release

FROM gcr.io/distroless/static-debian12:nonroot
WORKDIR /app
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/pdfgen-rs /app/pdfgen-rs

EXPOSE 8080
CMD ["/app/pdfgen-rs"]
