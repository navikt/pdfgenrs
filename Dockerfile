FROM clux/muslrust:stable as chef
RUN cargo install cargo-chef --locked --version 0.1.77
WORKDIR /build

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /build/recipe.json recipe.json
ENV RUSTFLAGS='-C target-feature=+crt-static'
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/static-debian12:nonroot
WORKDIR /app
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/pdfgenrs /app/pdfgenrs

EXPOSE 8080
CMD ["/app/pdfgenrs"]
