window.BENCHMARK_DATA = {
  "lastUpdate": 1790512617049,
  "repoUrl": "https://github.com/navikt/pdfgenrs",
  "entries": {
    "Criterion Benchmark": [
      {
        "commit": {
          "author": {
            "email": "joakimkartveit@gmail.com",
            "name": "Joakim Taule Kartveit",
            "username": "MikAoJk"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "24d69b3b38f7c3f5709e4d5b7c155bc5211c2fd3",
          "message": "Merge pull request #487 from navikt/copilot/validate-clamp-max-concurrent-compilations\n\nValidate semaphore permit configuration",
          "timestamp": "2026-09-19T13:30:57+02:00",
          "tree_id": "6aae74ec1e9051833d3be98f236f442c0fe0e37b",
          "url": "https://github.com/navikt/pdfgenrs/commit/24d69b3b38f7c3f5709e4d5b7c155bc5211c2fd3"
        },
        "date": 1789817660377,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 150569,
            "range": "± 8016",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 330531,
            "range": "± 3379",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4377643,
            "range": "± 26472",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 79123,
            "range": "± 6904",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6771522,
            "range": "± 42534",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 117042,
            "range": "± 4933",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 168133,
            "range": "± 6667",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 366893,
            "range": "± 4660",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 12125,
            "range": "± 147",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18284,
            "range": "± 25",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joakimkartveit@gmail.com",
            "name": "Joakim Taule Kartveit",
            "username": "MikAoJk"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fccbdce5852dcb1ed8ef98cdcf716e79991c0699",
          "message": "Merge pull request #488 from navikt/copilot/review-testutil-module\n\nHide test helpers from the public library API",
          "timestamp": "2026-09-20T16:43:27+02:00",
          "tree_id": "8fa79a3808ed1801c6fb5f5807143ebb9db55647",
          "url": "https://github.com/navikt/pdfgenrs/commit/fccbdce5852dcb1ed8ef98cdcf716e79991c0699"
        },
        "date": 1789915608817,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 125265,
            "range": "± 3989",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 277640,
            "range": "± 13687",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 3684950,
            "range": "± 148405",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 64418,
            "range": "± 5347",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6574555,
            "range": "± 267363",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 97856,
            "range": "± 5705",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 173865,
            "range": "± 9797",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 307738,
            "range": "± 7243",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 9831,
            "range": "± 242",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 15095,
            "range": "± 854",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joakimkartveit@gmail.com",
            "name": "Joakim Taule Kartveit",
            "username": "MikAoJk"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4ca4d818b6abc25288ff018cd16d7f12a0d5f4b0",
          "message": "Merge pull request #489 from navikt/copilot/add-startup-validation-image-limit\n\nWarn on conflicting image limits",
          "timestamp": "2026-09-21T07:18:40+02:00",
          "tree_id": "b318184d870e3bca264520676d26cb69089274c9",
          "url": "https://github.com/navikt/pdfgenrs/commit/4ca4d818b6abc25288ff018cd16d7f12a0d5f4b0"
        },
        "date": 1789968127549,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 164373,
            "range": "± 8222",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 311855,
            "range": "± 1343",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4172197,
            "range": "± 26084",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 67211,
            "range": "± 6678",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6401539,
            "range": "± 73936",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 128586,
            "range": "± 2622",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 168671,
            "range": "± 5269",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 337187,
            "range": "± 1329",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 8405,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 13964,
            "range": "± 42",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joakimkartveit@gmail.com",
            "name": "Joakim Taule Kartveit",
            "username": "MikAoJk"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f75ae2435b5f01118309bf8146a27aa69cf3b17f",
          "message": "Merge pull request #491 from navikt/bumpdeps\n\nchore: bump some deps",
          "timestamp": "2026-09-24T13:12:20+02:00",
          "tree_id": "81975baa3baef872b4380768da3bac70c0c89b39",
          "url": "https://github.com/navikt/pdfgenrs/commit/f75ae2435b5f01118309bf8146a27aa69cf3b17f"
        },
        "date": 1790248561850,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 189389,
            "range": "± 8447",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 397411,
            "range": "± 2011",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5385883,
            "range": "± 74545",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91811,
            "range": "± 6852",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 11194119,
            "range": "± 242025",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 146698,
            "range": "± 7305",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 210345,
            "range": "± 5572",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 428644,
            "range": "± 6340",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 14386,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 21469,
            "range": "± 235",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joakimkartveit@gmail.com",
            "name": "Joakim Taule Kartveit",
            "username": "MikAoJk"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "877badae2e4a6d23f8dca58d360f4e25cb9258e4",
          "message": "Merge pull request #492 from navikt/copilot/public-facing-modules-typed-errors\n\nReplace public `anyhow::Result` in exported APIs with typed error enums",
          "timestamp": "2026-09-24T18:51:34+02:00",
          "tree_id": "0371828571a9ebda6ab201a72ca6e130e2f9911f",
          "url": "https://github.com/navikt/pdfgenrs/commit/877badae2e4a6d23f8dca58d360f4e25cb9258e4"
        },
        "date": 1790268911717,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 196687,
            "range": "± 12590",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 354538,
            "range": "± 19615",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 6012619,
            "range": "± 423087",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 81934,
            "range": "± 8951",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 9954901,
            "range": "± 576618",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 149158,
            "range": "± 9098",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 197823,
            "range": "± 13296",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 345254,
            "range": "± 13015",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 9095,
            "range": "± 671",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 14278,
            "range": "± 421",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joakimkartveit@gmail.com",
            "name": "Joakim Taule Kartveit",
            "username": "MikAoJk"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ac1b0fae97db202979dec4863f1b5063476ae993",
          "message": "Merge pull request #493 from navikt/concurrency\n\nchore: added concurrency cancellation for workflows",
          "timestamp": "2026-09-25T10:07:53+02:00",
          "tree_id": "c925d2b0a440ae213c55f49aec581ef377020510",
          "url": "https://github.com/navikt/pdfgenrs/commit/ac1b0fae97db202979dec4863f1b5063476ae993"
        },
        "date": 1790323887909,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 164138,
            "range": "± 1085",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 310621,
            "range": "± 1765",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4163967,
            "range": "± 31095",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 67291,
            "range": "± 3775",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6179028,
            "range": "± 94633",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 124710,
            "range": "± 2575",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 168219,
            "range": "± 3586",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 336885,
            "range": "± 7897",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 8391,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 14113,
            "range": "± 68",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joakimkartveit@gmail.com",
            "name": "Joakim Taule Kartveit",
            "username": "MikAoJk"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3518f5f3e4ca751e107f5bda185ea69efb142db9",
          "message": "Merge pull request #494 from navikt/dependabot/cargo/ironpress-1.7.0\n\nchore(deps): bump ironpress from 1.6.0 to 1.7.0",
          "timestamp": "2026-09-25T15:33:46+02:00",
          "tree_id": "3bd2bca502c72bff9e2bd695ee4d0bc4ec8b7993",
          "url": "https://github.com/navikt/pdfgenrs/commit/3518f5f3e4ca751e107f5bda185ea69efb142db9"
        },
        "date": 1790343723617,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 210637,
            "range": "± 4150",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 399293,
            "range": "± 1710",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5338942,
            "range": "± 42290",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 86718,
            "range": "± 6798",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10985867,
            "range": "± 25678",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 148727,
            "range": "± 3823",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 214184,
            "range": "± 4622",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 436349,
            "range": "± 11683",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10866,
            "range": "± 237",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18195,
            "range": "± 55",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joakimkartveit@gmail.com",
            "name": "Joakim Taule Kartveit",
            "username": "MikAoJk"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "06248fdde747cd3f81627f040c21dc3f4c1bee57",
          "message": "Merge pull request #496 from navikt/copilot/pin-benchmark-workflow-to-1-98-1\n\nPin benchmark workflow to Rust 1.98.1",
          "timestamp": "2026-09-26T17:09:16+02:00",
          "tree_id": "87bead97329003563a6a6805a9525492dc8fdd15",
          "url": "https://github.com/navikt/pdfgenrs/commit/06248fdde747cd3f81627f040c21dc3f4c1bee57"
        },
        "date": 1790435592810,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 228244,
            "range": "± 2692",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 424099,
            "range": "± 9643",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5288903,
            "range": "± 60158",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91758,
            "range": "± 7902",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10432605,
            "range": "± 134449",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 177955,
            "range": "± 4374",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 222716,
            "range": "± 9147",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 451406,
            "range": "± 2688",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10991,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18864,
            "range": "± 273",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joakimkartveit@gmail.com",
            "name": "Joakim Taule Kartveit",
            "username": "MikAoJk"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "32e3a5570d0e83fc9d5151a6d3c44bd6d9d85782",
          "message": "Merge pull request #500 from navikt/copilot/improve-performance\n\nReuse prevalidated image dimensions in image PDF generation path",
          "timestamp": "2026-09-27T08:56:07+02:00",
          "tree_id": "cc4253137033536ee86bd122d9f83d709b8b33ea",
          "url": "https://github.com/navikt/pdfgenrs/commit/32e3a5570d0e83fc9d5151a6d3c44bd6d9d85782"
        },
        "date": 1790492803143,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 232852,
            "range": "± 16077",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 425660,
            "range": "± 13774",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5318839,
            "range": "± 116949",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91432,
            "range": "± 8464",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10358882,
            "range": "± 134179",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 174340,
            "range": "± 5448",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 223962,
            "range": "± 4772",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 452208,
            "range": "± 2403",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10924,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18910,
            "range": "± 120",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joakimkartveit@gmail.com",
            "name": "Joakim Taule Kartveit",
            "username": "MikAoJk"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "67943e4be6a261654ccc53de0027bf71824c7680",
          "message": "Merge pull request #502 from navikt/copilot/add-focused-tests-for-coverage\n\nAdd request-id middleware tests for conflicting response headers",
          "timestamp": "2026-09-27T14:34:02+02:00",
          "tree_id": "92b204fd6f0096540954483c7c7bfae69adce2e4",
          "url": "https://github.com/navikt/pdfgenrs/commit/67943e4be6a261654ccc53de0027bf71824c7680"
        },
        "date": 1790512609946,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 228984,
            "range": "± 15811",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 422073,
            "range": "± 5398",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5360530,
            "range": "± 52102",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91256,
            "range": "± 7799",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10560242,
            "range": "± 119749",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 168874,
            "range": "± 5811",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 216045,
            "range": "± 5069",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 451120,
            "range": "± 1667",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11009,
            "range": "± 132",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18984,
            "range": "± 165",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}