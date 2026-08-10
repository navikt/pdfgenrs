window.BENCHMARK_DATA = {
  "lastUpdate": 1786352932800,
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
          "id": "26c2a88745ac44278bb84a6bd7a87e8913b01263",
          "message": "Merge pull request #406 from navikt/copilot/remove-manual-content-length-header\n\nRemove redundant Content-Length header from pdf_response",
          "timestamp": "2026-08-09T14:49:15+02:00",
          "tree_id": "44a012da36319fa0e26084a0ff377b715437ccd8",
          "url": "https://github.com/navikt/pdfgenrs/commit/26c2a88745ac44278bb84a6bd7a87e8913b01263"
        },
        "date": 1786279887807,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 229289,
            "range": "± 9012",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 425690,
            "range": "± 3672",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5214892,
            "range": "± 56298",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91751,
            "range": "± 6264",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 157228,
            "range": "± 4176",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 221309,
            "range": "± 9379",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 362383,
            "range": "± 12941",
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
          "id": "28124db14d9ce7804d03010501ca0a91ff955165",
          "message": "Merge pull request #407 from navikt/copilot/replace-safety-comment-in-mod-rs\n\nRemove misused `// SAFETY:` comment from non-unsafe code",
          "timestamp": "2026-08-09T17:26:25+02:00",
          "tree_id": "47c95565c65c23595c1c1321d2e33d40790db909",
          "url": "https://github.com/navikt/pdfgenrs/commit/28124db14d9ce7804d03010501ca0a91ff955165"
        },
        "date": 1786289319973,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 216110,
            "range": "± 18771",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 403534,
            "range": "± 3941",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5286454,
            "range": "± 42813",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 87789,
            "range": "± 6234",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 157407,
            "range": "± 3763",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 214879,
            "range": "± 4656",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 358258,
            "range": "± 3225",
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
          "id": "07d4940e6a3ab969b4b850879fd5b7cf7b988e9f",
          "message": "Merge pull request #408 from navikt/copilot/pin-dockerfile-base-images\n\nPin Dockerfile base images to digests",
          "timestamp": "2026-08-09T17:44:03+02:00",
          "tree_id": "4dc4c1f628c7c8011c101470a789c84c8f10ff35",
          "url": "https://github.com/navikt/pdfgenrs/commit/07d4940e6a3ab969b4b850879fd5b7cf7b988e9f"
        },
        "date": 1786290375648,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 225251,
            "range": "± 12269",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 412749,
            "range": "± 2951",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5102349,
            "range": "± 20949",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91780,
            "range": "± 8161",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 153058,
            "range": "± 4926",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 206154,
            "range": "± 4050",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 355492,
            "range": "± 7296",
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
          "id": "cc0fe169ecb5578d030459adb7257094929030a4",
          "message": "Merge pull request #410 from navikt/copilot/replace-unreachable-with-apiexception\n\nReplace `unreachable!` panic in compile-permit acquisition with graceful ApiError",
          "timestamp": "2026-08-09T18:00:27+02:00",
          "tree_id": "a64690bb34a75817fb97eb9cf1cd86345d3f1544",
          "url": "https://github.com/navikt/pdfgenrs/commit/cc0fe169ecb5578d030459adb7257094929030a4"
        },
        "date": 1786291356200,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 165150,
            "range": "± 6391",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 307931,
            "range": "± 4359",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4085739,
            "range": "± 33243",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 67579,
            "range": "± 5544",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 115897,
            "range": "± 2526",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 166252,
            "range": "± 3507",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 272051,
            "range": "± 1468",
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
          "id": "be8fbea23e8604529f978f8865a715557cdc1c68",
          "message": "Merge pull request #409 from navikt/copilot/add-runtime-tests-for-env-values\n\nAdd runtime behavior test for zero request body limit",
          "timestamp": "2026-08-09T18:00:53+02:00",
          "tree_id": "efa29682501c7dfe870de98459f279ee47de39df",
          "url": "https://github.com/navikt/pdfgenrs/commit/be8fbea23e8604529f978f8865a715557cdc1c68"
        },
        "date": 1786291382879,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 228687,
            "range": "± 14997",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 428725,
            "range": "± 15616",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5131797,
            "range": "± 45651",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91966,
            "range": "± 8469",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 163191,
            "range": "± 3928",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 214995,
            "range": "± 6111",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 361082,
            "range": "± 2774",
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
          "id": "7fb2b8311644d112671dc16332dbb685e6d8c065",
          "message": "Merge pull request #411 from navikt/copilot/cargo-auditable-build-release-fix\n\nUse --locked with cargo auditable build in Dockerfile",
          "timestamp": "2026-08-09T18:22:53+02:00",
          "tree_id": "b9463f05fe695e104efb48ab2c457223ab048b3c",
          "url": "https://github.com/navikt/pdfgenrs/commit/7fb2b8311644d112671dc16332dbb685e6d8c065"
        },
        "date": 1786292711487,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 213498,
            "range": "± 10074",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 401449,
            "range": "± 3672",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5302712,
            "range": "± 64680",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 87437,
            "range": "± 8778",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 152541,
            "range": "± 2115",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 208423,
            "range": "± 2576",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 349960,
            "range": "± 7099",
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
          "id": "a55cdba002080d1a809a7caf163efa305b77f77c",
          "message": "Merge pull request #412 from navikt/copilot/add-image-to-pdf-benchmarks\n\nfeat: add HTML generation benchmarks to both benchmark harnesses",
          "timestamp": "2026-08-09T18:46:22+02:00",
          "tree_id": "2ea937f41bac57ee5409dc6c436ee776cadbf5a6",
          "url": "https://github.com/navikt/pdfgenrs/commit/a55cdba002080d1a809a7caf163efa305b77f77c"
        },
        "date": 1786294143447,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 232960,
            "range": "± 14015",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 424647,
            "range": "± 3760",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5234388,
            "range": "± 26062",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91943,
            "range": "± 7643",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 161150,
            "range": "± 3753",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 220482,
            "range": "± 9798",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 363313,
            "range": "± 1821",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11187,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 19045,
            "range": "± 98",
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
          "id": "fa8a71589d5063c6a1c4b96fb36001c34aaabd77",
          "message": "Merge pull request #415 from navikt/copilot/tune-comemo-cache-eviction\n\nAdd Prometheus metrics for comemo cache eviction tuning",
          "timestamp": "2026-08-10T08:23:57+02:00",
          "tree_id": "ee028cfccadb93388bcde69326fbee5cb7da3034",
          "url": "https://github.com/navikt/pdfgenrs/commit/fa8a71589d5063c6a1c4b96fb36001c34aaabd77"
        },
        "date": 1786343210883,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 178563,
            "range": "± 7681",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 392050,
            "range": "± 6346",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4971887,
            "range": "± 97697",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 97169,
            "range": "± 7033",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 124656,
            "range": "± 5658",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 198066,
            "range": "± 16229",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 327532,
            "range": "± 19823",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 14305,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 21395,
            "range": "± 65",
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
          "id": "4976341647975262f9c0936439bde648e232384c",
          "message": "Merge pull request #417 from navikt/copilot/fix-payload-size-observability\n\nRecord generated PDF response sizes",
          "timestamp": "2026-08-10T11:05:50+02:00",
          "tree_id": "3f9a12b7012e27280611e1d6bf8c989611901c5c",
          "url": "https://github.com/navikt/pdfgenrs/commit/4976341647975262f9c0936439bde648e232384c"
        },
        "date": 1786352917085,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 231874,
            "range": "± 13417",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 426965,
            "range": "± 4669",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5178815,
            "range": "± 31439",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92439,
            "range": "± 7189",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 165913,
            "range": "± 3438",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 214984,
            "range": "± 9397",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 366713,
            "range": "± 2563",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11171,
            "range": "± 149",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18980,
            "range": "± 86",
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
          "id": "9d9ce6fd6d2018cb99653b2d58af3f628f1dffce",
          "message": "Merge pull request #416 from navikt/copilot/timeouts-compile-management\n\nKeep compilation permits through timeouts",
          "timestamp": "2026-08-10T11:06:06+02:00",
          "tree_id": "a528e3687e7ce8d51020bf4e5128b9ecb8cc1fb2",
          "url": "https://github.com/navikt/pdfgenrs/commit/9d9ce6fd6d2018cb99653b2d58af3f628f1dffce"
        },
        "date": 1786352923998,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 231616,
            "range": "± 13903",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 425463,
            "range": "± 6657",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5160217,
            "range": "± 136040",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91890,
            "range": "± 9904",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 161429,
            "range": "± 4777",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 210729,
            "range": "± 7140",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 362220,
            "range": "± 6053",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10979,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 19252,
            "range": "± 151",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}