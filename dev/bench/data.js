window.BENCHMARK_DATA = {
  "lastUpdate": 1786428129301,
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
          "id": "6309e684b719ec3b7fbd45b1f797c9fa2710e5dc",
          "message": "Merge pull request #418 from navikt/copilot/set-default-concurrency-limit\n\nLimit default Typst compilation concurrency",
          "timestamp": "2026-08-10T12:16:52+02:00",
          "tree_id": "80d0249d40d9c0e2e2cfb7efa5046bb488591b0e",
          "url": "https://github.com/navikt/pdfgenrs/commit/6309e684b719ec3b7fbd45b1f797c9fa2710e5dc"
        },
        "date": 1786357181096,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 229308,
            "range": "± 5170",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 424044,
            "range": "± 2392",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5073661,
            "range": "± 71244",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 90902,
            "range": "± 6480",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 163379,
            "range": "± 4311",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 213009,
            "range": "± 8943",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 359970,
            "range": "± 4098",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10925,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18819,
            "range": "± 79",
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
          "id": "c3eafac76833e0791fd91ace05d976dc0bde4e4d",
          "message": "Merge pull request #419 from navikt/copilot/fix-fonts-dir-issue\n\nResolve relative font paths from ROOT_DIR at startup",
          "timestamp": "2026-08-10T17:29:41+02:00",
          "tree_id": "77236e717c4bda1f4cb51131cd0a3073c973db0c",
          "url": "https://github.com/navikt/pdfgenrs/commit/c3eafac76833e0791fd91ace05d976dc0bde4e4d"
        },
        "date": 1786375951401,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 192890,
            "range": "± 8136",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 396492,
            "range": "± 3341",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5157741,
            "range": "± 59341",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92012,
            "range": "± 4138",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 134496,
            "range": "± 4414",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 201389,
            "range": "± 7546",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 338611,
            "range": "± 1843",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 14207,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 21414,
            "range": "± 74",
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
          "id": "659896722d07cbcd2350f4362780e7e56c5f5eeb",
          "message": "Merge pull request #420 from navikt/copilot/add-validated-dimension-limits\n\nLimit decoded image dimensions before Typst compilation",
          "timestamp": "2026-08-10T18:06:20+02:00",
          "tree_id": "470ebaf02b00eccc4fa05f01687438558edbc320",
          "url": "https://github.com/navikt/pdfgenrs/commit/659896722d07cbcd2350f4362780e7e56c5f5eeb"
        },
        "date": 1786378132575,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 214201,
            "range": "± 9461",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 398337,
            "range": "± 9458",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5175812,
            "range": "± 24479",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 87290,
            "range": "± 7735",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 149207,
            "range": "± 2013",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 206160,
            "range": "± 4944",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 356017,
            "range": "± 1477",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10962,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18386,
            "range": "± 60",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joakimkartveit@gmail.com",
            "name": "MikAoJk",
            "username": "MikAoJk"
          },
          "committer": {
            "email": "joakimkartveit@gmail.com",
            "name": "MikAoJk",
            "username": "MikAoJk"
          },
          "distinct": true,
          "id": "65a45f16defda9f74536966ff0d38960ff0e2c65",
          "message": "chore: fixed typo",
          "timestamp": "2026-08-11T07:59:15+02:00",
          "tree_id": "951f091186f91a3874f1b225d74c35b122dafd44",
          "url": "https://github.com/navikt/pdfgenrs/commit/65a45f16defda9f74536966ff0d38960ff0e2c65"
        },
        "date": 1786428120536,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 237068,
            "range": "± 15773",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 428477,
            "range": "± 5246",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5252503,
            "range": "± 45171",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 93309,
            "range": "± 6876",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 166541,
            "range": "± 6229",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 216924,
            "range": "± 8405",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 367176,
            "range": "± 1991",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10801,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18392,
            "range": "± 70",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}