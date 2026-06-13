window.BENCHMARK_DATA = {
  "lastUpdate": 1781355217553,
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
          "id": "755f6532fb1ccba26b0e1bbe99c02221db9c7847",
          "message": "Merge pull request #254 from navikt/copilot/html-report-interactivity\n\nImplementing interactivity for HTML report in GitHub Actions",
          "timestamp": "2026-06-08T08:14:28+02:00",
          "tree_id": "37390f1cffae1c6b6cd17fe5d7cfd1ba3af10fa9",
          "url": "https://github.com/navikt/pdfgenrs/commit/755f6532fb1ccba26b0e1bbe99c02221db9c7847"
        },
        "date": 1780901004223,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 755392,
            "range": "± 16903",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1053764,
            "range": "± 48418",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1168877980,
            "range": "± 1913469",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 575517,
            "range": "± 8054",
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
          "id": "0c81d8767409023a8b719ee06d5c76865fb10de5",
          "message": "Merge pull request #255 from navikt/copilot/image-dimension-parsing-consistency\n\nUnify bounds checking pattern in image dimension parsers",
          "timestamp": "2026-06-08T11:19:36+02:00",
          "tree_id": "081f3ce440e141b403dce6622fa0403e448920cb",
          "url": "https://github.com/navikt/pdfgenrs/commit/0c81d8767409023a8b719ee06d5c76865fb10de5"
        },
        "date": 1780910883549,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 830883,
            "range": "± 9819",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1069037,
            "range": "± 18185",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1168785197,
            "range": "± 1364569",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 666537,
            "range": "± 10051",
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
          "id": "bd45dec3c220572d53f45387eb8933497b315140",
          "message": "Merge pull request #256 from navikt/copilot/refactor-acquire-compile-permit\n\nrefactor: extract shared compile helper to eliminate route handler duplication",
          "timestamp": "2026-06-08T11:47:13+02:00",
          "tree_id": "1c448612dc2010470544fffa67ad28fe7e13b4e7",
          "url": "https://github.com/navikt/pdfgenrs/commit/bd45dec3c220572d53f45387eb8933497b315140"
        },
        "date": 1780912334370,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 847820,
            "range": "± 52879",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1074068,
            "range": "± 24303",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1169731544,
            "range": "± 1026805",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 674954,
            "range": "± 9505",
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
          "id": "afbc060c196d403c29bc1faf3d1ebfbca2447a3a",
          "message": "Merge pull request #257 from navikt/copilot/full-html-report-github-pages\n\nDeploy full Criterion HTML report to GitHub Pages",
          "timestamp": "2026-06-08T11:54:18+02:00",
          "tree_id": "eeb6563ba7e77143cb008357c8eb5fe8146490b5",
          "url": "https://github.com/navikt/pdfgenrs/commit/afbc060c196d403c29bc1faf3d1ebfbca2447a3a"
        },
        "date": 1780912739511,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 850776,
            "range": "± 26859",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1080973,
            "range": "± 8947",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1167074173,
            "range": "± 527708",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 675438,
            "range": "± 10413",
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
            "email": "joakimkartveit@gmail.com",
            "name": "Joakim Taule Kartveit",
            "username": "MikAoJk"
          },
          "distinct": true,
          "id": "7afe8dac7972a1b1980df4ccf12d69de4740bca8",
          "message": "chore: remove #[must_use",
          "timestamp": "2026-06-08T12:15:56+02:00",
          "tree_id": "904e7ff56471f5cd1f9f49f77e7a05d283d52007",
          "url": "https://github.com/navikt/pdfgenrs/commit/7afe8dac7972a1b1980df4ccf12d69de4740bca8"
        },
        "date": 1780914101319,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 597712,
            "range": "± 16064",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 821103,
            "range": "± 56709",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1166225201,
            "range": "± 779805",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 447144,
            "range": "± 8776",
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
          "id": "dcafe793a97cdaae18ad03c4250375867907b9ac",
          "message": "Merge pull request #258 from navikt/copilot/hashmap-single-entry-construction\n\nRefactoring HashMap single-entry construction",
          "timestamp": "2026-06-08T12:21:54+02:00",
          "tree_id": "3e7c4ce93dc3eca8b9e3d30b4452572d2207d901",
          "url": "https://github.com/navikt/pdfgenrs/commit/dcafe793a97cdaae18ad03c4250375867907b9ac"
        },
        "date": 1780914398762,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 765405,
            "range": "± 16558",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1057036,
            "range": "± 33715",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1169532970,
            "range": "± 1517380",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 580426,
            "range": "± 17523",
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
          "id": "57a7d3d223b22b07bf70ac15b7e84ab8f9163e21",
          "message": "Merge pull request #259 from navikt/copilot/pre-allocate-hashmap-in-typst-to-pdf\n\nPre-allocate HashMap in typst_to_pdf",
          "timestamp": "2026-06-08T14:22:13+02:00",
          "tree_id": "758b6529e63b06a9e56addd79a0641697d8790e6",
          "url": "https://github.com/navikt/pdfgenrs/commit/57a7d3d223b22b07bf70ac15b7e84ab8f9163e21"
        },
        "date": 1780921606922,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 852633,
            "range": "± 65181",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1077380,
            "range": "± 21445",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1169178800,
            "range": "± 1398723",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 676292,
            "range": "± 8322",
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
          "id": "9059f082feae35258f634883c4b2f23af657d374",
          "message": "Merge pull request #260 from navikt/copilot/avoid-string-allocation\n\nrefactor: avoid unnecessary String allocation in typst_to_pdf/typst_to_html",
          "timestamp": "2026-06-08T14:30:38+02:00",
          "tree_id": "d9dfdd7fe46486f07c81282b410fdb950a6dc969",
          "url": "https://github.com/navikt/pdfgenrs/commit/9059f082feae35258f634883c4b2f23af657d374"
        },
        "date": 1780922120074,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 841923,
            "range": "± 8391",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1079052,
            "range": "± 10776",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1167788934,
            "range": "± 1335933",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 674167,
            "range": "± 12545",
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
          "id": "d55a586692aa9cddfc1f023b2b05bea5e1fe312a",
          "message": "Merge pull request #261 from navikt/copilot/extract-common-route-handler-logic\n\nExtract common route handler logic into helper functions",
          "timestamp": "2026-06-08T14:59:36+02:00",
          "tree_id": "228097a6cac11091d74aff1deea914064787894a",
          "url": "https://github.com/navikt/pdfgenrs/commit/d55a586692aa9cddfc1f023b2b05bea5e1fe312a"
        },
        "date": 1780923864868,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 794793,
            "range": "± 30006",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1068758,
            "range": "± 36026",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1173490232,
            "range": "± 4692660",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 629655,
            "range": "± 8385",
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
          "id": "91937e1910d97ca4721b763afac2b4c60080e5e0",
          "message": "Merge pull request #264 from navikt/copilot/check-unit-tests-coverage\n\nAdd unit tests for compile_blocking timeout, semaphore concurrency, and fallback handler",
          "timestamp": "2026-06-10T13:00:12+02:00",
          "tree_id": "790e618f773e52f45b33861105afd80c72efd070",
          "url": "https://github.com/navikt/pdfgenrs/commit/91937e1910d97ca4721b763afac2b4c60080e5e0"
        },
        "date": 1781089485899,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 799214,
            "range": "± 6068",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1063354,
            "range": "± 22028",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1168986293,
            "range": "± 789306",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 620479,
            "range": "± 17845",
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
          "id": "89b1fdcc94bd4a298afae8ab92f6a6a38dbd2a71",
          "message": "Merge pull request #262 from navikt/MikAoJk-patch-1\n\nchore: tighten Clippy lint compliance",
          "timestamp": "2026-06-10T14:24:45+02:00",
          "tree_id": "fc268902308142a3e0dd8d1bb695de59e9331b6c",
          "url": "https://github.com/navikt/pdfgenrs/commit/89b1fdcc94bd4a298afae8ab92f6a6a38dbd2a71"
        },
        "date": 1781094565147,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 776391,
            "range": "± 19332",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1064866,
            "range": "± 11924",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1170049177,
            "range": "± 1575565",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 582103,
            "range": "± 20444",
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
          "id": "d30f32c2fb3db09f175ae2ac6c5631b605043449",
          "message": "Merge pull request #266 from navikt/copilot/add-request-response-size-metrics\n\nAdd request/response body size histogram metrics",
          "timestamp": "2026-06-10T15:01:08+02:00",
          "tree_id": "632e369aa35a1709c3d2bdeab1fc35143ef27955",
          "url": "https://github.com/navikt/pdfgenrs/commit/d30f32c2fb3db09f175ae2ac6c5631b605043449"
        },
        "date": 1781096748540,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 809458,
            "range": "± 8302",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1033813,
            "range": "± 12267",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1177614051,
            "range": "± 1048436",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 645345,
            "range": "± 13813",
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
          "id": "31866e0d6f91713c13f19686e5fdaa0b52511bde",
          "message": "Merge pull request #265 from navikt/copilot/add-rfc-9457-uris\n\nImplementing RFC 9457 problem type URIs",
          "timestamp": "2026-06-10T15:00:54+02:00",
          "tree_id": "94e05931cb6e15e57a4b8837ce61fa98929a0ae5",
          "url": "https://github.com/navikt/pdfgenrs/commit/31866e0d6f91713c13f19686e5fdaa0b52511bde"
        },
        "date": 1781096750522,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 848205,
            "range": "± 33717",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1076490,
            "range": "± 22725",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1167356495,
            "range": "± 851252",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 677149,
            "range": "± 12595",
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
          "id": "8345228e695a988fcb29387bfffe96c824a5342f",
          "message": "Merge pull request #267 from navikt/copilot/add-content-length-header\n\nAdd Content-Length header to PDF/HTML responses",
          "timestamp": "2026-06-10T15:08:26+02:00",
          "tree_id": "cc62bc280ed3d67e1da57c09b96056d67a6fb20f",
          "url": "https://github.com/navikt/pdfgenrs/commit/8345228e695a988fcb29387bfffe96c824a5342f"
        },
        "date": 1781097220291,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 802299,
            "range": "± 36188",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1084132,
            "range": "± 25436",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1171896479,
            "range": "± 926631",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 631151,
            "range": "± 14756",
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
          "id": "4fe640896ae632b16294c941f4d1f7b3dce96d3f",
          "message": "Merge pull request #268 from navikt/copilot/cache-html-converter-font-aliases\n\nCache HTML converter font alias bytes using OnceLock",
          "timestamp": "2026-06-10T17:14:02+02:00",
          "tree_id": "38b7cebf638a8e0d0230c78ad43fa2852cf61028",
          "url": "https://github.com/navikt/pdfgenrs/commit/4fe640896ae632b16294c941f4d1f7b3dce96d3f"
        },
        "date": 1781104771646,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 856516,
            "range": "± 7940",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1081252,
            "range": "± 13399",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1168966893,
            "range": "± 1966126",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 676353,
            "range": "± 14219",
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
          "id": "fa0a1c720b3d66e91651ce8a9ef8370cf6f4d51c",
          "message": "Merge pull request #270 from navikt/copilot/semaphore-timeout-on-acquisition\n\nAdd semaphore acquisition timeout with 503 Service Unavailable response",
          "timestamp": "2026-06-10T18:51:07+02:00",
          "tree_id": "daf2bdba0325de1d3b013955627773b806e1adb8",
          "url": "https://github.com/navikt/pdfgenrs/commit/fa0a1c720b3d66e91651ce8a9ef8370cf6f4d51c"
        },
        "date": 1781110908095,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 846969,
            "range": "± 18691",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1086081,
            "range": "± 12330",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1167976109,
            "range": "± 1352688",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 677085,
            "range": "± 12987",
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
            "email": "joakimkartveit@gmail.com",
            "name": "Joakim Taule Kartveit",
            "username": "MikAoJk"
          },
          "distinct": true,
          "id": "6b5bd25578b8cd4411a0f6c209a1abebbf8e3943",
          "message": "chore: bumped deps",
          "timestamp": "2026-06-10T21:20:20+02:00",
          "tree_id": "138f5c2ea5a6f667f6e4ce3722276f2a2905746a",
          "url": "https://github.com/navikt/pdfgenrs/commit/6b5bd25578b8cd4411a0f6c209a1abebbf8e3943"
        },
        "date": 1781119452832,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 771073,
            "range": "± 34050",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1059510,
            "range": "± 15213",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1168124259,
            "range": "± 670662",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 586765,
            "range": "± 24363",
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
          "id": "083d47642dcb905b6b5656309bfcf51cf49f9bbb",
          "message": "Merge pull request #273 from navikt/copilot/drop-failed-load-font-log\n\nDowngrade font alias file-not-found log from warn to debug",
          "timestamp": "2026-06-11T10:07:38+02:00",
          "tree_id": "b3ac743ce1e54112f6c508d3c73ec649c32da35d",
          "url": "https://github.com/navikt/pdfgenrs/commit/083d47642dcb905b6b5656309bfcf51cf49f9bbb"
        },
        "date": 1781165470386,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 848840,
            "range": "± 22737",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1083675,
            "range": "± 15835",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1166378999,
            "range": "± 718871",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 675871,
            "range": "± 12093",
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
          "id": "584c0fd5f5f61707ad80c116e916a9a9580584bf",
          "message": "Merge pull request #276 from navikt/revert-273-copilot/drop-failed-load-font-log\n\nRevert \"Downgrade font alias file-not-found log from warn to debug\"",
          "timestamp": "2026-06-11T10:23:41+02:00",
          "tree_id": "19dc1f7599cc416fc5ed0250a3edb1107b907345",
          "url": "https://github.com/navikt/pdfgenrs/commit/584c0fd5f5f61707ad80c116e916a9a9580584bf"
        },
        "date": 1781166432460,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 855079,
            "range": "± 73941",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1095226,
            "range": "± 61204",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1166925326,
            "range": "± 444119",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 692393,
            "range": "± 18598",
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
          "id": "0fa34652a046a2d60d1ddecf559b1a8276610408",
          "message": "Merge pull request #274 from navikt/MikAoJk-patch-1\n\nRemove obsolete font aliases from HTML_FONT_ALIASES",
          "timestamp": "2026-06-11T10:24:06+02:00",
          "tree_id": "4d4b96a3d0e6333a303f358aef83571fbe679811",
          "url": "https://github.com/navikt/pdfgenrs/commit/0fa34652a046a2d60d1ddecf559b1a8276610408"
        },
        "date": 1781166446568,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 749377,
            "range": "± 40864",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1056822,
            "range": "± 25926",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1168466850,
            "range": "± 695974",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 578448,
            "range": "± 15686",
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
          "id": "dd47be8606e59e5f94196bb81fa4d13dd2f92e08",
          "message": "Merge pull request #277 from navikt/copilot/support-pdfa-2a-pdfua-1\n\nfeat: support both PDF/A-2a and PDF/UA-1 by upgrading typst to 0.15.0-rc.1",
          "timestamp": "2026-06-11T11:59:03+02:00",
          "tree_id": "b801370851fdab923e377bef8a4f4e212b2305a9",
          "url": "https://github.com/navikt/pdfgenrs/commit/dd47be8606e59e5f94196bb81fa4d13dd2f92e08"
        },
        "date": 1781172449454,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 868280,
            "range": "± 23746",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1078017,
            "range": "± 12515",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1166783835,
            "range": "± 862480",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 772549,
            "range": "± 5730",
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
          "id": "8905204d25f2ce06c5e65fd87f997b39474c6fff",
          "message": "Merge pull request #279 from navikt/dependabot/github_actions/github/codeql-action-4.36.2\n\nchore(deps): bump github/codeql-action from 4.36.0 to 4.36.2",
          "timestamp": "2026-06-12T15:24:36+02:00",
          "tree_id": "6a947f4189bc22d9d489df3498460126848d4aa8",
          "url": "https://github.com/navikt/pdfgenrs/commit/8905204d25f2ce06c5e65fd87f997b39474c6fff"
        },
        "date": 1781270889979,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 871152,
            "range": "± 20258",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1088217,
            "range": "± 14248",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1170970094,
            "range": "± 736315",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 773911,
            "range": "± 9654",
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
          "id": "9db2d87631be5c848a65655d70ef0af5909c1c32",
          "message": "Merge pull request #281 from navikt/dependabot/github_actions/actions/upload-artifact-7\n\nchore(deps): bump actions/upload-artifact from 4 to 7",
          "timestamp": "2026-06-12T15:24:55+02:00",
          "tree_id": "04f8c4c4f6f628a95995b7d5a8c60924dfe5601c",
          "url": "https://github.com/navikt/pdfgenrs/commit/9db2d87631be5c848a65655d70ef0af5909c1c32"
        },
        "date": 1781270901524,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 867746,
            "range": "± 21012",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1074827,
            "range": "± 6379",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1168423025,
            "range": "± 811937",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 762353,
            "range": "± 4763",
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
          "id": "94d778d5d208ddebfb09c6066b40d88bec0152ea",
          "message": "Merge pull request #280 from navikt/dependabot/github_actions/actions/checkout-6.0.3\n\nchore(deps): bump actions/checkout from 6.0.2 to 6.0.3",
          "timestamp": "2026-06-12T15:24:46+02:00",
          "tree_id": "3207884716a4e572e71715935c25dfcac0b170eb",
          "url": "https://github.com/navikt/pdfgenrs/commit/94d778d5d208ddebfb09c6066b40d88bec0152ea"
        },
        "date": 1781270917818,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 877294,
            "range": "± 60379",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1086512,
            "range": "± 40801",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1169246136,
            "range": "± 931204",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 790032,
            "range": "± 14954",
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
          "id": "601fd39ba667da79d231b24fa6da7b2a40997a16",
          "message": "Merge pull request #282 from navikt/copilot/add-inline-to-small-functions\n\nAdd #[inline] to hot-path small functions",
          "timestamp": "2026-06-12T20:46:01+02:00",
          "tree_id": "29ce0205f442ee28f534284fc43c033a691f3b30",
          "url": "https://github.com/navikt/pdfgenrs/commit/601fd39ba667da79d231b24fa6da7b2a40997a16"
        },
        "date": 1781290164572,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 870988,
            "range": "± 6018",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1106293,
            "range": "± 18660",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1171026225,
            "range": "± 1404273",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 773228,
            "range": "± 10673",
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
          "id": "991c3e47caad1b4625b01f3af7181a3a0aff4fce",
          "message": "Merge pull request #283 from navikt/copilot/reduce-repeated-pathbuf-cloning\n\nReduce repeated PathBuf cloning in route handlers by wrapping paths in Arc",
          "timestamp": "2026-06-13T08:34:15+02:00",
          "tree_id": "3b56810e5702578be54831ca96fa43a009aca00e",
          "url": "https://github.com/navikt/pdfgenrs/commit/991c3e47caad1b4625b01f3af7181a3a0aff4fce"
        },
        "date": 1781332674480,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 876398,
            "range": "± 39202",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1109384,
            "range": "± 16755",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1168633549,
            "range": "± 831105",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 778064,
            "range": "± 27492",
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
          "id": "91b6e5e905f52983e8812a53e68b514d5dddae95",
          "message": "Merge pull request #284 from navikt/MikAoJk-patch-1\n\nchore: Add comment to explain offset decomposition",
          "timestamp": "2026-06-13T14:49:52+02:00",
          "tree_id": "205aab71cfc075e0dc37edef1d15c48cf1907c6f",
          "url": "https://github.com/navikt/pdfgenrs/commit/91b6e5e905f52983e8812a53e68b514d5dddae95"
        },
        "date": 1781355214365,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 873446,
            "range": "± 19237",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1108306,
            "range": "± 6910",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1167728366,
            "range": "± 610793",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 780258,
            "range": "± 12407",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}