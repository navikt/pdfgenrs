FROM clux/muslrust:stable as builder

WORKDIR /build
COPY . .
ENV RUSTFLAGS='-C target-feature=+crt-static'
ENV DISABLE_PDF_GET="true"
ENV ENABLE_HTML_ENDPOINT="false"

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        chromium \
        ca-certificates \
        fontconfig \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/pdfgenrs /app/pdfgenrs
COPY fonts /app/fonts

# Register /app/fonts with fontconfig so Chromium can find custom fonts.
# Child images that add their own fonts only need to run: RUN fc-cache -f
RUN printf '<?xml version="1.0"?>\n<!DOCTYPE fontconfig SYSTEM "fonts.dtd">\n<fontconfig>\n  <dir>/app/fonts</dir>\n</fontconfig>\n' \
        > /etc/fonts/conf.d/99-pdfgen-fonts.conf && \
    fc-cache -f

RUN useradd --system --no-create-home --shell /usr/sbin/nologin --uid 1000 appuser
USER appuser

EXPOSE 8080
CMD ["/app/pdfgenrs"]
