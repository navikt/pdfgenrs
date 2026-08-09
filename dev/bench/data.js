window.BENCHMARK_DATA = {
  "lastUpdate": 1786291390618,
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
          "id": "dbe3293ca49a9506a48d61954417afad76d76f86",
          "message": "Merge pull request #400 from navikt/copilot/request-tracing-correlation\n\nfix: propagate request ID into Typst compilation span",
          "timestamp": "2026-08-09T08:25:59+02:00",
          "tree_id": "e7b539ea886bdffd62b3ff5ab7462b1c47b3a2a6",
          "url": "https://github.com/navikt/pdfgenrs/commit/dbe3293ca49a9506a48d61954417afad76d76f86"
        },
        "date": 1786256895672,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 235904,
            "range": "± 17236",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 433167,
            "range": "± 4439",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5676945,
            "range": "± 148211",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92396,
            "range": "± 6709",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 158820,
            "range": "± 4341",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 212454,
            "range": "± 8984",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 367859,
            "range": "± 2792",
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
          "id": "2bbb9f631b0fd9fa847ba65f3a0b323f2e822800",
          "message": "Merge pull request #401 from navikt/copilot/image-format-validation\n\nfix: validate image bytes match declared Content-Type before passing to Typst",
          "timestamp": "2026-08-09T09:02:37+02:00",
          "tree_id": "2e5823470f30b33b0e6c4ff4604737aace0a435f",
          "url": "https://github.com/navikt/pdfgenrs/commit/2bbb9f631b0fd9fa847ba65f3a0b323f2e822800"
        },
        "date": 1786259084987,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 226987,
            "range": "± 9898",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 413843,
            "range": "± 3790",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5010217,
            "range": "± 29679",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 90956,
            "range": "± 7401",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 155225,
            "range": "± 4799",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 212635,
            "range": "± 7818",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 357172,
            "range": "± 2052",
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
          "id": "2dd1588e24d1efb8f36b7c43812eeadce60ffd4f",
          "message": "Merge pull request #403 from navikt/MikAoJk-patch-1\n\nchore: Add --release to clippy to share build artifacts with tests",
          "timestamp": "2026-08-09T09:46:39+02:00",
          "tree_id": "8821c38c6271f1f43ea7486168bcd4fa92b56248",
          "url": "https://github.com/navikt/pdfgenrs/commit/2dd1588e24d1efb8f36b7c43812eeadce60ffd4f"
        },
        "date": 1786261725942,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 217050,
            "range": "± 10908",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 404683,
            "range": "± 40288",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5251614,
            "range": "± 155262",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 88073,
            "range": "± 4954",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 150309,
            "range": "± 3824",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 215806,
            "range": "± 3647",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 361630,
            "range": "± 7506",
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
          "id": "a6ee9e4a540a042447bceff08b3686c2a427bd76",
          "message": "Merge pull request #404 from navikt/copilot/review-repository-enhancements-d90a5655-e108-43c4-b559-171db4ff3d5a\n\nperf: eliminate redundant format detection and heap allocation in image processing",
          "timestamp": "2026-08-09T14:37:41+02:00",
          "tree_id": "f12c23f6c2c9663cbbbf5489e0947b11a51ac1dc",
          "url": "https://github.com/navikt/pdfgenrs/commit/a6ee9e4a540a042447bceff08b3686c2a427bd76"
        },
        "date": 1786279192753,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 180614,
            "range": "± 6541",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 399767,
            "range": "± 4287",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5040674,
            "range": "± 47714",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 94379,
            "range": "± 5936",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 127128,
            "range": "± 7852",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 197341,
            "range": "± 7547",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 327506,
            "range": "± 3562",
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
          "id": "28424238bafa08a0c132b5fd80f8321deb0ef118",
          "message": "Merge pull request #405 from navikt/copilot/add-logging-on-timeout\n\nAdd warn log on compile_blocking timeout",
          "timestamp": "2026-08-09T14:42:43+02:00",
          "tree_id": "7d1f5c98958c4c0cdc87d98bc8510c44f20adb39",
          "url": "https://github.com/navikt/pdfgenrs/commit/28424238bafa08a0c132b5fd80f8321deb0ef118"
        },
        "date": 1786279502041,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 190614,
            "range": "± 6209",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 398264,
            "range": "± 4859",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5174719,
            "range": "± 26206",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 93891,
            "range": "± 5643",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 134397,
            "range": "± 5207",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 201381,
            "range": "± 5826",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 326721,
            "range": "± 4353",
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
      }
    ]
  }
}