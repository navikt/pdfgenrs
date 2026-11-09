# Performance tests

## Prerequisites

Install the following tools and verify that they are available:

| Tool | Check installation | Installation guide |
| --- | --- | --- |
| Rust | `rustc --version` | [Rust setup](../README.md#Rust) |
| Docker | `docker --version` | [Install Docker](https://docs.docker.com/engine/install/) |
| curl | `curl --version` | [Install curl](https://curl.se/docs/install.html) |
| k6 | `k6 --version` | [Install k6](https://grafana.com/docs/k6/latest/set-up/install-k6/) |

## Running k6 performance-test
1. Create docker image of pdfgenrs
```bash
docker build -t pdfgenrs:test .
docker run --rm --name pdfgenrs-dev \
  -p 8080:8080 \
  -v "$(pwd)/templates:/app/templates:ro" \
  -v "$(pwd)/data:/app/data:ro" \
  -v "$(pwd)/resources:/app/resources:ro" \
  -e DEV_MODE=true \
  pdfgenrs:test
```



2. Then you can run the performance test
```bash script
cd performance-test
k6 run -e BASE_URL=http://localhost:8080 script.js
```