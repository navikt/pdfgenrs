FROM clux/muslrust:stable@sha256:01ce6650c2b57796fe9a883df50e7d1254ef75209914d7f25c86d31bf1465abf AS builder
RUN cargo install cargo-auditable
WORKDIR /build
ENV RUSTFLAGS='-C target-feature=+crt-static'
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY fonts ./fonts
RUN cargo auditable build --release --locked

FROM gcr.io/distroless/static-debian13:nonroot@sha256:1c2c046bc09ed40fad370b599a0b1ae7987f55b01e247cf27a7c27cd97e5bbc7
WORKDIR /app
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/pdfgenrs /app/pdfgenrs
COPY --from=builder /build/fonts /app/fonts

EXPOSE 8080
CMD ["/app/pdfgenrs"]
