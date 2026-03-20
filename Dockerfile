FROM clux/muslrust:stable as builder

WORKDIR /build
COPY . .
ENV RUSTFLAGS='-C target-feature=+crt-static'
ENV DISABLE_PDF_GET="true"
ENV ENABLE_HTML_ENDPOINT="false"

RUN cargo build --release

FROM gcr.io/distroless/static-debian12:nonroot
WORKDIR /app
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/pdfgenrs /app/pdfgenrs

EXPOSE 8080
CMD ["/app/pdfgenrs"]
