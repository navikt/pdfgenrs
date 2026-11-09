import http from 'k6/http';
import { check, fail, sleep } from 'k6';
import exec from 'k6/execution';
import { Counter, Rate, Trend } from 'k6/metrics';

function positiveInteger(name, fallback) {
  const value = Number(__ENV[name] || fallback);
  if (!Number.isSafeInteger(value) || value <= 0) {
    throw new Error(`${name} must be a positive integer`);
  }
  return value;
}

const stageSeconds = positiveInteger('STAGE_SECONDS', 10);
const maxVUs = positiveInteger('MAX_VUS', 8);
const maxP95 = positiveInteger('MAX_P95_MS', 100);
const baseUrl = (__ENV.BASE_URL || 'http://localhost:8080').replace(/\/+$/, '');
const fixtures = [
  ['nav-brev', 'brev'],
  ['pale-2', 'pale-2'],
  ['syfosoknader', 'arbeidstakere'],
  ['dsopkontroll', 'kontrollforespoersel'],
  ['bench', 'html-bench'],
].map(([app, template]) => ({
  operation: `${app}/${template}`,
  path: `/api/v1/genpdf/${app}/${template}`,
  body: open(`../data/${app}/${template}.json`),
  contentType: 'application/json',
}));
const htmlData = JSON.parse(open('../data/bench/html-bench.json'));
const largeData = {
  title: 'Large JSON performance test',
  body: 'A report containing 100 items.',
  items: Array.from({ length: 100 }, (_, index) => ({
    name: `Item ${index}`,
    value: `Description for item ${index}`,
  })),
};
const images = [
  ['png', '../resources/NAVLogoRed.png', 'image/png'],
  ['jpeg', '../resources/NAVLogoRed.jpg', 'image/jpeg'],
  ['svg', '../resources/pdfgenrs-logo.svg', 'image/svg+xml'],
  ['raster-2048', './fixtures/2048.png', 'image/png'],
  ['raster-4096', './fixtures/4096.png', 'image/png'],
].map(([name, path, contentType]) => ({
  operation: `image-${name}`,
  path: '/api/v1/genpdf/image/k6',
  body: open(path, 'b'),
  contentType,
  uniquePng: name.startsWith('raster-'),
}));
const workloads = [
  ...fixtures,
  ...images,
  {
    operation: 'html-to-pdf',
    path: '/api/v1/genpdf/html/k6',
    contentType: 'text/html',
    body: '<!DOCTYPE html><html><head><style>body { font-family: "Source Sans 3", sans-serif; }</style></head>'
      + '<body><h1>HTML to PDF</h1><p>Performance test document.</p></body></html>',
  },
  {
    operation: 'typst-to-html',
    path: '/api/v1/genhtml/bench/html-bench',
    contentType: 'application/json',
    body: JSON.stringify(htmlData),
    expectedHtml: htmlData.title,
  },
  ...['genpdf', 'genhtml'].map((endpoint) => ({
    operation: `large-json-${endpoint}`,
    path: `/api/v1/${endpoint}/bench/html-bench`,
    contentType: 'application/json',
    body: JSON.stringify(largeData),
    expectedHtml: endpoint === 'genhtml' ? largeData.title : undefined,
  })),
];
const thresholds = {
  checks: ['rate==1'],
  http_req_failed: ['rate==0'],
  api_resource_samples: ['count>0'],
};
for (const workload of workloads) {
  const tags = `operation:${workload.operation},scenario:rendering`;
  thresholds[`http_req_duration{${tags}}`] = [`p(95)<${maxP95}`];
  thresholds[`rendered_documents{${tags}}`] = ['count>0'];
}

export const options = {
  scenarios: {
    rendering: {
      executor: 'ramping-vus',
      startVUs: 1,
      stages: [
        { duration: `${stageSeconds}s`, target: 1 },
        { duration: `${stageSeconds}s`, target: Math.max(1, Math.floor(maxVUs / 2)) },
        { duration: `${stageSeconds}s`, target: maxVUs },
        { duration: `${stageSeconds}s`, target: 0 },
      ],
      gracefulRampDown: '30s',
    },
    resources: {
      executor: 'constant-vus',
      vus: 1,
      duration: `${stageSeconds * 4}s`,
      exec: 'sampleResources',
    },
  },
  summaryTrendStats: ['min', 'avg', 'max', 'med', 'p(90)', 'p(95)', 'p(99)'],
  thresholds,
};

const documents = new Counter('rendered_documents');
const resourceSamples = new Counter('api_resource_samples');
const inFlight = new Trend('api_compile_in_flight');
const saturated = new Rate('api_compile_saturated');
let processStart;

function uniquePng(fixture, unique) {
  const text = `k6_request\0${unique}`;
  const chunk = new Uint8Array(text.length + 12);
  const view = new DataView(chunk.buffer);
  view.setUint32(0, text.length);
  chunk.set([116, 69, 88, 116], 4);
  for (let index = 0; index < text.length; index++) {
    chunk[index + 8] = text.charCodeAt(index);
  }
  let crc = 0xffffffff;
  for (const byte of chunk.subarray(4, -4)) {
    crc ^= byte;
    for (let bit = 0; bit < 8; bit++) {
      crc = (crc >>> 1) ^ ((crc & 1) ? 0xedb88320 : 0);
    }
  }
  view.setUint32(chunk.length - 4, (crc ^ 0xffffffff) >>> 0);
  const image = new Uint8Array(fixture);
  const result = new Uint8Array(image.length + chunk.length);
  result.set(image.subarray(0, -12));
  result.set(chunk, image.length - 12);
  result.set(image.subarray(-12), result.length - 12);
  return result.buffer;
}

function render(workload, unique) {
  let body = workload.body;
  if (workload.contentType === 'application/json') {
    const data = JSON.parse(body);
    data.k6_request_id = unique;
    if (workload.path.endsWith('/bench/html-bench')) {
      data.body = `${data.body} ${unique}`;
    }
    body = JSON.stringify(data);
  } else if (workload.contentType === 'text/html') {
    body = body.replace('</body>', `<p>${unique}</p></body>`);
  } else if (workload.uniquePng) {
    body = uniquePng(body, unique);
  }
  const response = http.post(`${baseUrl}${workload.path}`, body, {
    headers: { 'Content-Type': workload.contentType },
    tags: { operation: workload.operation, name: workload.path },
    responseType: workload.expectedHtml ? 'text' : 'binary',
    timeout: '45s',
  });
  const valid = check(response, {
    [`${workload.operation}: status 200`]: (res) => res.status === 200,
    [`${workload.operation}: valid document`]: (res) => {
      const contentType = res.headers['Content-Type'] || '';
      if (workload.expectedHtml) {
        return contentType.startsWith('text/html')
          && typeof res.body === 'string'
          && res.body.includes(workload.expectedHtml)
          && res.body.includes('</html>');
      }
      if (!contentType.startsWith('application/pdf') || !(res.body instanceof ArrayBuffer)) {
        return false;
      }
      const bytes = new Uint8Array(res.body);
      const header = String.fromCharCode(...bytes.slice(0, 5));
      const trailer = String.fromCharCode(...bytes.slice(-32)).trimEnd();
      return header === '%PDF-' && trailer.endsWith('%%EOF');
    },
  });
  if (!valid) {
    fail(`${workload.operation} failed: HTTP ${response.status}, error=${response.error}`);
  }
  documents.add(1, { operation: workload.operation });
}

export function setup() {
  const response = http.get(`${baseUrl}/internal/is_ready`, { timeout: '5s' });
  if (!check(response, { 'service is ready': (res) => res.status === 200 })) {
    fail(`Service is not ready at ${baseUrl}`);
  }
  for (const workload of workloads) {
    render(workload, 'preflight');
  }
}

export default function () {
  const unique = `${exec.vu.idInTest}-${exec.scenario.iterationInTest}`;
  for (const workload of workloads) {
    render(workload, unique);
  }
}

function metric(body, name) {
  const match = body.match(new RegExp(`^${name}\\s+(\\S+)`, 'm'));
  const value = match ? Number(match[1]) : NaN;
  if (!check(value, {
    [`metrics: valid ${name}`]: (sample) => Number.isFinite(sample) && sample >= 0,
  })) {
    fail(`Missing or invalid Prometheus metric: ${name}`);
  }
  return value;
}

export function sampleResources() {
  try {
    const response = http.get(`${baseUrl}/internal/metrics`, {
      tags: { operation: 'metrics' },
      responseType: 'text',
      timeout: '5s',
    });
    if (!check(response, { 'metrics: status 200': (res) => res.status === 200 })) {
      fail(`Metrics request failed: HTTP ${response.status}`);
    }
    const start = metric(response.body, 'process_start_time_seconds');
    const available = metric(response.body, 'pdfgenrs_compile_permits_available');
    const total = metric(response.body, 'pdfgenrs_compile_permits_total');
    const active = metric(response.body, 'pdfgenrs_compile_in_flight');
    const valid = check(null, {
      'metrics: valid compilation permits': () =>
        start > 0 && Number.isInteger(total) && total > 0
        && Number.isInteger(available) && available <= total
        && Number.isInteger(active) && active <= total,
      'metrics: same process': () =>
        processStart === undefined || start === processStart,
    });
    if (!valid) {
      fail('Invalid resource sample, disabled compilation limit, or API process restarted');
    }
    processStart = start;
    inFlight.add(active);
    saturated.add(available === 0);
    resourceSamples.add(1);
  } finally {
    sleep(1);
  }
}
