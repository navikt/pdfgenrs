# Performance tests

### Prerequisites
1. [rust](../README.md#Rust)
2. docker

Make sure you have docker installed using this command:
```bash script
docker --version
```
or install it: https://docs.docker.com/engine/install/

3. curl
   
Make sure you have curl installed using this command:
```bash script
curl --version
```
or install it: https://curl.se/docs/install.html   

4. k6

Make sure you have k6 installed using this command:
```bash script
k6 --version
```
or install it: https://grafana.com/docs/k6/latest/set-up/install-k6/


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


### What is run during the performance test

The test first checks `/internal/is_ready` and renders every workload once as a
preflight check. During load, each virtual user repeatedly runs these 14 rendering
operations sequentially:

| Workload | Operations |
| --- | --- |
| JSON to PDF | `nav-brev/brev`, `pale-2/pale-2`, `syfosoknader/arbeidstakere`, `dsopkontroll/kontrollforespoersel`, and `bench/html-bench` |
| Image to PDF | PNG, JPEG, SVG, and 2048px and 4096px raster PNG fixtures |
| HTML to PDF | An HTML document with inline CSS |
| Typst to HTML | `bench/html-bench` |
| Large JSON to PDF and HTML | A document containing 100 items, rendered to both formats |

JSON requests include a unique request ID. The benchmark JSON body, HTML input,
and large raster PNG payloads also vary between iterations to exercise changing
inputs rather than only identical requests.

The default load has four 10-second stages: hold at 1 virtual user, ramp to 4,
ramp to 8, then ramp down to 0. In-flight iterations have a 30-second graceful
ramp-down allowance. A separate virtual user samples compilation metrics and
process start time from `/internal/metrics` roughly once per second during the
40-second load period.

#### Pass/fail thresholds

| Metric | Requirement |
| --- | --- |
| Request duration per rendering operation | `p(95)<100`: the 95th-percentile HTTP request duration must be strictly below 100 ms |
| Checks | `rate==1`: all readiness, response, document, and metrics checks must pass |
| HTTP request failures | `rate==0`: no failed HTTP requests |
| Rendering coverage | `count>0`: every operation must produce at least one valid document during the load scenario |
| Resource sampling | `count>0`: at least one valid resource sample must be collected |

Latency thresholds apply separately to each rendering operation in the load
scenario, excluding preflight and metrics requests. Fast operations cannot hide
a slow operation. Request duration measures HTTP time, not compilation time alone.

Rendering checks require HTTP 200 and the expected content type. PDFs must have
the `%PDF-` header and `%%EOF` trailer; HTML must contain the expected title and
closing `</html>` tag.

Active compilations and compilation saturation are reported but have no
upper-limit thresholds. Resource checks require valid metrics, an enabled
compilation limit, and an unchanged process start time. The performance test does
not collect CPU or memory measurements. There is no minimum throughput threshold
or comparison against previous runs.

#### Configuration

| Environment variable | Default | Purpose |
| --- | --- | --- |
| `BASE_URL` | `http://localhost:8080` | Server to test |
| `STAGE_SECONDS` | `10` | Duration of each of the four load stages |
| `MAX_VUS` | `8` | Peak number of rendering virtual users; the intermediate target is half this value, rounded down, with a minimum of 1 |
| `MAX_P95_MS` | `100` | Strict upper bound for each operation's p95 request duration in milliseconds |

Numeric settings must be positive integers. For example, from this directory:

```bash
k6 run -e MAX_P95_MS=1500 -e MAX_VUS=16 -e STAGE_SECONDS=20 script.js
```