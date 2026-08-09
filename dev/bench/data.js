window.BENCHMARK_DATA = {
  "lastUpdate": 1786289326765,
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
          "id": "ef2bc274250fd708a3e97a9dec139d9cef17f5a7",
          "message": "Merge pull request #399 from navikt/copilot/fix-vp8l-signature-verification\n\nfix: verify VP8L signature byte before parsing dimensions",
          "timestamp": "2026-08-09T07:54:25+02:00",
          "tree_id": "4b9b98c7545c358156057f1ee8fa382995303a0a",
          "url": "https://github.com/navikt/pdfgenrs/commit/ef2bc274250fd708a3e97a9dec139d9cef17f5a7"
        },
        "date": 1786255012689,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 217262,
            "range": "± 7211",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 399998,
            "range": "± 2973",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5185188,
            "range": "± 111295",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 87820,
            "range": "± 9000",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 152011,
            "range": "± 3347",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 211743,
            "range": "± 11733",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 362762,
            "range": "± 2183",
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
          "id": "483c55dd5c04621c0628081b0d62ec98dcc2e903",
          "message": "Merge pull request #398 from navikt/copilot/propagate-opentelemetry-span-context\n\nPropagate OTel span context into spawn_blocking in compile_blocking",
          "timestamp": "2026-08-09T07:55:10+02:00",
          "tree_id": "e9d65de6b5dbe72ec8f151acb58764a5cdcc19ea",
          "url": "https://github.com/navikt/pdfgenrs/commit/483c55dd5c04621c0628081b0d62ec98dcc2e903"
        },
        "date": 1786255047905,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 232372,
            "range": "± 15769",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 418005,
            "range": "± 15579",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5172025,
            "range": "± 177951",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92595,
            "range": "± 9150",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 160729,
            "range": "± 4427",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 211669,
            "range": "± 7379",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 359594,
            "range": "± 7355",
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
          "id": "d2e35009a0c4670198551f3dfa3e06889347113f",
          "message": "Merge pull request #397 from navikt/copilot/jpeg-dimensions-fix-robustness-gap\n\nExtend jpeg_dimensions to cover SOF9–SOF11 arithmetic-coded JPEG markers",
          "timestamp": "2026-08-09T07:55:28+02:00",
          "tree_id": "e1d75969ef4bf2284a38ada1c98306b917bfcdf3",
          "url": "https://github.com/navikt/pdfgenrs/commit/d2e35009a0c4670198551f3dfa3e06889347113f"
        },
        "date": 1786255054575,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 149162,
            "range": "± 2544",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 335429,
            "range": "± 4787",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4286224,
            "range": "± 53147",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 80458,
            "range": "± 5924",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 112500,
            "range": "± 4790",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 164379,
            "range": "± 7764",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 277083,
            "range": "± 2689",
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
      }
    ]
  }
}