window.BENCHMARK_DATA = {
  "lastUpdate": 1783499040176,
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
          "id": "874df8be8dbabfb9cd54f7f9b3b25641655d2242",
          "message": "Merge pull request #285 from navikt/copilot/force-use-1900-or-higher\n\nAdd rust-version = \"1.90.0\" to enforce minimum Rust version",
          "timestamp": "2026-06-13T17:31:43+02:00",
          "tree_id": "d17535062a4e5f14d4e0048b589d7acc24b37d59",
          "url": "https://github.com/navikt/pdfgenrs/commit/874df8be8dbabfb9cd54f7f9b3b25641655d2242"
        },
        "date": 1781364914939,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 877472,
            "range": "± 20591",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1110672,
            "range": "± 6080",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1167536676,
            "range": "± 743952",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 780240,
            "range": "± 6895",
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
          "id": "25ce42c76cafe93cbabf4ce5e860ee59d0534533",
          "message": "Merge pull request #286 from navikt/MikAoJk-patch-1\n\nUpdate Rust version in Cargo.toml",
          "timestamp": "2026-06-13T18:01:22+02:00",
          "tree_id": "0a7af1b85d53f3be8462098d87d62035dca58a62",
          "url": "https://github.com/navikt/pdfgenrs/commit/25ce42c76cafe93cbabf4ce5e860ee59d0534533"
        },
        "date": 1781366687402,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 875828,
            "range": "± 19546",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1109875,
            "range": "± 14885",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1169711624,
            "range": "± 633430",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 776429,
            "range": "± 16831",
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
          "id": "82c0eadede13df16b7f57ac474d3f0ded30f113c",
          "message": "Merge pull request #287 from navikt/copilot/verify-fonts-and-templates-loaded\n\nEnhance is_ready endpoint to verify critical resources are loaded",
          "timestamp": "2026-06-14T17:00:22+02:00",
          "tree_id": "f0ae3f6597aac273ce567aa7f3eed7a2579ed7b0",
          "url": "https://github.com/navikt/pdfgenrs/commit/82c0eadede13df16b7f57ac474d3f0ded30f113c"
        },
        "date": 1781449425344,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 869512,
            "range": "± 37901",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1105210,
            "range": "± 28394",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1167982170,
            "range": "± 445662",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 771131,
            "range": "± 4223",
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
          "id": "1ac65c3951f05a44c79d2d3c3b6d24fce8d82d5f",
          "message": "Merge pull request #288 from navikt/copilot/update-typst-version-0-15-0\n\nUpdate typst version from 0.15.0-rc.1 to 0.15.0",
          "timestamp": "2026-06-16T07:29:48+02:00",
          "tree_id": "73e105dac1a8614f8a889212e64f6ffcf44d85d5",
          "url": "https://github.com/navikt/pdfgenrs/commit/1ac65c3951f05a44c79d2d3c3b6d24fce8d82d5f"
        },
        "date": 1781588083825,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 877732,
            "range": "± 21308",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1114492,
            "range": "± 15169",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1167363963,
            "range": "± 644712",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 779402,
            "range": "± 10967",
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
          "id": "d5886e7ec8b944555ac7e8861c40fdd9d80758a6",
          "message": "Merge pull request #289 from navikt/bump\n\nchore: bumped tower-http version",
          "timestamp": "2026-06-16T20:29:44+02:00",
          "tree_id": "921cdf1df96a53cac91279657bb42df57df8eea6",
          "url": "https://github.com/navikt/pdfgenrs/commit/d5886e7ec8b944555ac7e8861c40fdd9d80758a6"
        },
        "date": 1781634822762,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 867311,
            "range": "± 23212",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1077443,
            "range": "± 7670",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1166610317,
            "range": "± 799896",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 769152,
            "range": "± 12593",
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
          "id": "f393b72c248db6ab5ff4dc04afb0cb4ec8fea0df",
          "message": "Merge pull request #291 from navikt/copilot/create-unit-tests\n\nAdd unit tests for image parsers, symlink handling, HTTP flows, and config edge cases",
          "timestamp": "2026-06-19T08:26:23+02:00",
          "tree_id": "423ceed0408f4c174cd747a80d3bd39a7f6bad91",
          "url": "https://github.com/navikt/pdfgenrs/commit/f393b72c248db6ab5ff4dc04afb0cb4ec8fea0df"
        },
        "date": 1781850594488,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 863725,
            "range": "± 73559",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1068074,
            "range": "± 16252",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1168005140,
            "range": "± 705805",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 764130,
            "range": "± 4656",
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
          "id": "f10a23f822f4a213a1b3967703c250afd2da380c",
          "message": "Merge pull request #293 from navikt/MikAoJk-patch-1\n\nchore: Add more lint warnings",
          "timestamp": "2026-06-19T09:49:16+02:00",
          "tree_id": "69e21d01387de2ed398871b0cf71621f81bde66b",
          "url": "https://github.com/navikt/pdfgenrs/commit/f10a23f822f4a213a1b3967703c250afd2da380c"
        },
        "date": 1781855568639,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 864114,
            "range": "± 52886",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1074568,
            "range": "± 39353",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1165959298,
            "range": "± 462395",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 769474,
            "range": "± 9604",
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
          "id": "bbf69f87fda3d202b9ab51cd1cab143b98afa401",
          "message": "Merge pull request #296 from navikt/copilot/check-unused-code-swap-libs\n\nReplace chrono with time crate",
          "timestamp": "2026-06-22T19:45:04+02:00",
          "tree_id": "a8ffd9b148b392497a6e3559c5ce1a1763d1e127",
          "url": "https://github.com/navikt/pdfgenrs/commit/bbf69f87fda3d202b9ab51cd1cab143b98afa401"
        },
        "date": 1782150617192,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 854805,
            "range": "± 21942",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1069730,
            "range": "± 13616",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1167723103,
            "range": "± 779041",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 773163,
            "range": "± 12502",
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
          "id": "e29321fc0230e50c01afbb5769a84caede67e785",
          "message": "Merge pull request #297 from navikt/copilot/image-format-support\n\nAdd WebP and SVG image format support and upgrade dependencies",
          "timestamp": "2026-06-25T08:56:57+02:00",
          "tree_id": "e96a8dca3f2bcea90d0f5ce8a78de2c75c35a066",
          "url": "https://github.com/navikt/pdfgenrs/commit/e29321fc0230e50c01afbb5769a84caede67e785"
        },
        "date": 1782371195671,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 871172,
            "range": "± 38598",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1106965,
            "range": "± 5052",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1167761574,
            "range": "± 674336",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 777312,
            "range": "± 4562",
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
          "id": "e2b4f08b01fbdab317c999f9df805a1a038c5056",
          "message": "Merge pull request #299 from navikt/copilot/add-startup-warning-degenerate-values\n\nAdd startup warnings for degenerate configuration values",
          "timestamp": "2026-06-25T11:11:29+02:00",
          "tree_id": "68135589685855b5a1f4e5f09ca55bfa0b8823d4",
          "url": "https://github.com/navikt/pdfgenrs/commit/e2b4f08b01fbdab317c999f9df805a1a038c5056"
        },
        "date": 1782378909530,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 871627,
            "range": "± 40393",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1108706,
            "range": "± 5344",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1170203120,
            "range": "± 752365",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 769448,
            "range": "± 6648",
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
          "id": "31a18a57899ce49cad95d48f2d9967d258933b45",
          "message": "Update criterion-benchmark.yml",
          "timestamp": "2026-06-25T14:35:24+02:00",
          "tree_id": "c991078ba3f555086ef3b71e1f1cb8bde0751498",
          "url": "https://github.com/navikt/pdfgenrs/commit/31a18a57899ce49cad95d48f2d9967d258933b45"
        },
        "date": 1782391159679,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 676479,
            "range": "± 3386",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 858918,
            "range": "± 10135",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1165457823,
            "range": "± 643448",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 629776,
            "range": "± 4294",
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
          "id": "e566723a8dd1d5d0a93e56ebb59368c38f772173",
          "message": "Merge pull request #300 from navikt/MikAoJk-patch-1\n\nUpdate project description in Cargo.toml",
          "timestamp": "2026-06-26T08:17:41+02:00",
          "tree_id": "4f538f3c4fba6583c1ad2e076a02115de16ca04f",
          "url": "https://github.com/navikt/pdfgenrs/commit/e566723a8dd1d5d0a93e56ebb59368c38f772173"
        },
        "date": 1782454874236,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 867131,
            "range": "± 12246",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1084794,
            "range": "± 8236",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1166704255,
            "range": "± 2023851",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 774130,
            "range": "± 5285",
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
          "id": "32065eab218e0f3030f172c2d4b7fecb7b1359e3",
          "message": "Merge pull request #301 from navikt/dependabot/github_actions/release-drafter/release-drafter-7.4.0\n\nchore(deps): bump release-drafter/release-drafter from 7.3.1 to 7.4.0",
          "timestamp": "2026-06-26T18:33:27+02:00",
          "tree_id": "d45d2eb16b6ffdacdbe8c6e32b7090038606f3d2",
          "url": "https://github.com/navikt/pdfgenrs/commit/32065eab218e0f3030f172c2d4b7fecb7b1359e3"
        },
        "date": 1782491834502,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 870172,
            "range": "± 19388",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1082406,
            "range": "± 15498",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1170291272,
            "range": "± 1270169",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 810340,
            "range": "± 13736",
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
          "id": "5ab7428a8ad56f09e44daf65e25c9cf878328364",
          "message": "Merge pull request #302 from navikt/dependabot/github_actions/actions/checkout-7.0.0\n\nchore(deps): bump actions/checkout from 6.0.3 to 7.0.0",
          "timestamp": "2026-06-26T18:33:43+02:00",
          "tree_id": "6f3474dbe0b3cf813959d8a46efc6025675d9774",
          "url": "https://github.com/navikt/pdfgenrs/commit/5ab7428a8ad56f09e44daf65e25c9cf878328364"
        },
        "date": 1782491854346,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 861698,
            "range": "± 36095",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1073184,
            "range": "± 10140",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1169757162,
            "range": "± 1163607",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 770567,
            "range": "± 6643",
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
          "id": "db3ba9d929dfc9a7a3f1d0d0759fb3d79f52b3ab",
          "message": "Merge pull request #305 from navikt/copilot/expand-integration-tests-for-error-scenarios\n\nExpanding integration tests for error scenarios in routes",
          "timestamp": "2026-06-29T12:31:29+02:00",
          "tree_id": "5eeb70ad45d22f8101167fa58cbad69804ae653f",
          "url": "https://github.com/navikt/pdfgenrs/commit/db3ba9d929dfc9a7a3f1d0d0759fb3d79f52b3ab"
        },
        "date": 1782729300282,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 869635,
            "range": "± 19150",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1076852,
            "range": "± 11595",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1167377433,
            "range": "± 937553",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 777036,
            "range": "± 8291",
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
          "id": "48701e1b8d9cc659d480691bb585be681b3c2d05",
          "message": "Merge pull request #304 from navikt/MikAoJk-patch-1\n\nAdd new lints for panic_in_result_fn and cognitive_complexity",
          "timestamp": "2026-06-29T12:46:44+02:00",
          "tree_id": "253ddd0ecff0fe46b3bc093fd9eba520fca0458c",
          "url": "https://github.com/navikt/pdfgenrs/commit/48701e1b8d9cc659d480691bb585be681b3c2d05"
        },
        "date": 1782730237793,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 880145,
            "range": "± 40880",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1083355,
            "range": "± 15109",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1170226689,
            "range": "± 703405",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 785567,
            "range": "± 39409",
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
          "id": "d2bf5951a8cc9d7af600f9f4b5d1fb61ba300b6d",
          "message": "Merge pull request #306 from navikt/MikAoJk-patch-1\n\nUpdate lints in Cargo.toml for Clippy warnings",
          "timestamp": "2026-06-29T13:48:37+02:00",
          "tree_id": "f6c560ab8e75d62b2c56b255ee117710fab3eb9d",
          "url": "https://github.com/navikt/pdfgenrs/commit/d2bf5951a8cc9d7af600f9f4b5d1fb61ba300b6d"
        },
        "date": 1782733933888,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 872908,
            "range": "± 19219",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1110070,
            "range": "± 15355",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1168633664,
            "range": "± 573692",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 770325,
            "range": "± 8288",
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
          "id": "3f5c6ec7c6ad187c229ba2b6287a97e8da3c55bd",
          "message": "Merge pull request #307 from navikt/copilot/fix-integer-overflow-today-function\n\nfix: use saturating arithmetic in today() to prevent integer overflow",
          "timestamp": "2026-06-29T14:20:43+02:00",
          "tree_id": "34af4b6b035e3ae334fe40a3254bf6aa387868b2",
          "url": "https://github.com/navikt/pdfgenrs/commit/3f5c6ec7c6ad187c229ba2b6287a97e8da3c55bd"
        },
        "date": 1782735860165,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 870227,
            "range": "± 27175",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1082082,
            "range": "± 10125",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1166464295,
            "range": "± 750612",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 816973,
            "range": "± 11680",
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
          "id": "4cdafbaf4ffb1d97cbbca626658b88987ae39174",
          "message": "Merge pull request #308 from navikt/copilot/add-comment-to-semaphore-invariant\n\nDocument semaphore lifetime invariant at unreachable! site",
          "timestamp": "2026-06-29T14:47:30+02:00",
          "tree_id": "30c05bea2f3a267a61a1f558df20921ba7bf7d61",
          "url": "https://github.com/navikt/pdfgenrs/commit/4cdafbaf4ffb1d97cbbca626658b88987ae39174"
        },
        "date": 1782737467876,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 851726,
            "range": "± 5068",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 1066421,
            "range": "± 5048",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1173894060,
            "range": "± 1055784",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 746445,
            "range": "± 4680",
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
          "id": "9907b828a34a15209d3d8a56fda3c544d83836bc",
          "message": "Merge pull request #310 from navikt/copilot/optimize-pdf-generation\n\nrefactor: pre-build Typst Library once and share via Arc",
          "timestamp": "2026-06-29T16:46:46+02:00",
          "tree_id": "354e5becdc87f2325f935f81dd7ce768032fd448",
          "url": "https://github.com/navikt/pdfgenrs/commit/9907b828a34a15209d3d8a56fda3c544d83836bc"
        },
        "date": 1782744616523,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 193427,
            "range": "± 9439",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 402741,
            "range": "± 6862",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1175434303,
            "range": "± 813566",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 126864,
            "range": "± 3613",
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
          "id": "1fde640f223a10e9b93d419979d954afe3637085",
          "message": "Merge pull request #311 from navikt/copilot/fix-503-status-logs\n\nfix: suppress duplicate \"response failed\" logging from tower-http on 503",
          "timestamp": "2026-06-29T19:27:15+02:00",
          "tree_id": "6c03f0e4949dba5255b9b015e07ee2aa677d327d",
          "url": "https://github.com/navikt/pdfgenrs/commit/1fde640f223a10e9b93d419979d954afe3637085"
        },
        "date": 1782754249116,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 225459,
            "range": "± 4528",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 422403,
            "range": "± 11089",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1168372241,
            "range": "± 840382",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 152968,
            "range": "± 5505",
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
          "id": "0696f9790f7a912dbf49b57dddf00f91c3b6bdf0",
          "message": "Merge pull request #312 from navikt/cleanup\n\nchore: small clean up",
          "timestamp": "2026-06-29T20:33:03+02:00",
          "tree_id": "ccd81b5034e1884a01daf9d5d29590f5611be56a",
          "url": "https://github.com/navikt/pdfgenrs/commit/0696f9790f7a912dbf49b57dddf00f91c3b6bdf0"
        },
        "date": 1782758228075,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 222310,
            "range": "± 2745",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 417712,
            "range": "± 4591",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1270344783,
            "range": "± 1240584",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 144768,
            "range": "± 5371",
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
          "id": "17b0f514a549513311d80832d0d5c0d845989594",
          "message": "Merge pull request #313 from navikt/copilot/drop-unnecessary-code-line\n\nrefactor: replace hardcoded HTML_FONT_ALIASES with dynamic font discovery",
          "timestamp": "2026-06-30T08:33:42+02:00",
          "tree_id": "a860bf52970e0b51394c9e866d535b2c717eb437",
          "url": "https://github.com/navikt/pdfgenrs/commit/17b0f514a549513311d80832d0d5c0d845989594"
        },
        "date": 1782801454230,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 92031,
            "range": "± 3216",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 196688,
            "range": "± 1400",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1259763341,
            "range": "± 1346070",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 65081,
            "range": "± 5333",
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
          "id": "7b8c8c874d98a1288c9e6d328f3ef80a1da0cfee",
          "message": "Merge pull request #314 from navikt/copilot/fix-503-service-unavailable\n\nfix: exclude nais health check routes from tower-http trace layer",
          "timestamp": "2026-06-30T09:34:07+02:00",
          "tree_id": "479060406004ba5860aba53dd86c0a5526ad5f63",
          "url": "https://github.com/navikt/pdfgenrs/commit/7b8c8c874d98a1288c9e6d328f3ef80a1da0cfee"
        },
        "date": 1782805084147,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 225177,
            "range": "± 15003",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 415037,
            "range": "± 2249",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1269348144,
            "range": "± 659487",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 153563,
            "range": "± 3817",
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
          "id": "50401dbc1ed43d77c57f182a7095ff792eb6bd03",
          "message": "Add files via upload",
          "timestamp": "2026-06-30T10:12:22+02:00",
          "tree_id": "492e892e2651a0d761d9ca48a04dcab952730288",
          "url": "https://github.com/navikt/pdfgenrs/commit/50401dbc1ed43d77c57f182a7095ff792eb6bd03"
        },
        "date": 1782807389585,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 167106,
            "range": "± 1687",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 314762,
            "range": "± 5378",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1266218843,
            "range": "± 609549",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 114156,
            "range": "± 2770",
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
          "id": "1dde2444e286d75f2e44f4b9f1156b40771744d5",
          "message": "Merge pull request #318 from navikt/copilot/log-413-payload-response\n\nImplementing logging for HTTP response codes",
          "timestamp": "2026-07-02T13:59:24+02:00",
          "tree_id": "30b106ba797b03c193021856fbf10ef1edd0c037",
          "url": "https://github.com/navikt/pdfgenrs/commit/1dde2444e286d75f2e44f4b9f1156b40771744d5"
        },
        "date": 1782994246966,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 229688,
            "range": "± 11792",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 427019,
            "range": "± 2093",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1271004839,
            "range": "± 713037",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 157344,
            "range": "± 3158",
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
          "id": "b78c0c6c6cf42bf520426a553839ce85bd978056",
          "message": "Merge pull request #319 from navikt/copilot/verify-request-body-limit-bytes\n\ntest: verify REQUEST_BODY_LIMIT_BYTES works with custom values",
          "timestamp": "2026-07-02T14:37:39+02:00",
          "tree_id": "54ae7aaad761fa241e570afc68da401ac08d68b0",
          "url": "https://github.com/navikt/pdfgenrs/commit/b78c0c6c6cf42bf520426a553839ce85bd978056"
        },
        "date": 1782996534859,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 227643,
            "range": "± 13422",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 418893,
            "range": "± 2561",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1265987091,
            "range": "± 908459",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 149592,
            "range": "± 5282",
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
          "id": "688e6be609115ececd6c090358b54aa85f816393",
          "message": "Merge pull request #321 from navikt/copilot/update-dependencies-refresh-lockfile\n\nchore: update dependencies and refresh Cargo.lock",
          "timestamp": "2026-07-02T17:54:46+02:00",
          "tree_id": "3e214902f213ef79814019aed730d054537f06bb",
          "url": "https://github.com/navikt/pdfgenrs/commit/688e6be609115ececd6c090358b54aa85f816393"
        },
        "date": 1783008355119,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 215617,
            "range": "± 2149",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 401436,
            "range": "± 2398",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1269974266,
            "range": "± 881813",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 145575,
            "range": "± 2370",
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
          "id": "c896d566a3c1a581a5f80fb881a4dbfdf91d404d",
          "message": "Merge pull request #323 from navikt/copilot/fix-unused-constants\n\nGate linux-only test constants with #[cfg(target_os = \"linux\")]",
          "timestamp": "2026-07-02T18:25:58+02:00",
          "tree_id": "e7087a058bad7ef79fa861d43e5cb65b515dfdf5",
          "url": "https://github.com/navikt/pdfgenrs/commit/c896d566a3c1a581a5f80fb881a4dbfdf91d404d"
        },
        "date": 1783009788052,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 220995,
            "range": "± 5020",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 406811,
            "range": "± 4498",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1267374974,
            "range": "± 770436",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 143615,
            "range": "± 4267",
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
          "id": "8e3305a34752fb001e7819c605e968c03672dcf6",
          "message": "Merge pull request #325 from navikt/copilot/graceful-shutdown-bind-error-path\n\nfix: set alive=false when TCP bind fails during startup",
          "timestamp": "2026-07-03T07:19:22+02:00",
          "tree_id": "58f6d13aad188a38c943909204c02f8c58733341",
          "url": "https://github.com/navikt/pdfgenrs/commit/8e3305a34752fb001e7819c605e968c03672dcf6"
        },
        "date": 1783056206453,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 227648,
            "range": "± 1201",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 420577,
            "range": "± 2530",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1265350069,
            "range": "± 415269",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 156881,
            "range": "± 4034",
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
          "id": "1a435b9dea64fbab77f6f622f3690c57646a36b2",
          "message": "Merge pull request #329 from navikt/dependabot/github_actions/actions/cache-6.1.0\n\nchore(deps): bump actions/cache from 5.0.5 to 6.1.0",
          "timestamp": "2026-07-03T16:50:37+02:00",
          "tree_id": "d4ac3f2f2364531459bb2b7c8b0a789fbf1e9add",
          "url": "https://github.com/navikt/pdfgenrs/commit/1a435b9dea64fbab77f6f622f3690c57646a36b2"
        },
        "date": 1783090479027,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 229323,
            "range": "± 1514",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 422092,
            "range": "± 12677",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1265379681,
            "range": "± 797773",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 155466,
            "range": "± 3437",
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
          "id": "771a39704fc71dd0b87a742cd91c6110d0286e2f",
          "message": "Merge pull request #328 from navikt/dependabot/github_actions/release-drafter/release-drafter-7.5.1\n\nchore(deps): bump release-drafter/release-drafter from 7.4.0 to 7.5.1",
          "timestamp": "2026-07-03T16:50:50+02:00",
          "tree_id": "1c13be74215c701df037820ce3f157b82c20a8ab",
          "url": "https://github.com/navikt/pdfgenrs/commit/771a39704fc71dd0b87a742cd91c6110d0286e2f"
        },
        "date": 1783090493433,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 224363,
            "range": "± 2214",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 422396,
            "range": "± 2298",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1271287905,
            "range": "± 708297",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 145685,
            "range": "± 4436",
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
          "id": "045de8d69a86a22a8bb08e7e7bf94bdd6695b362",
          "message": "Merge pull request #327 from navikt/dependabot/cargo/axum-test-21.0.0\n\nchore(deps): bump axum-test from 20.1.0 to 21.0.0",
          "timestamp": "2026-07-03T16:51:02+02:00",
          "tree_id": "16c38a91f6352c17a0cad0d1477f6843e329e124",
          "url": "https://github.com/navikt/pdfgenrs/commit/045de8d69a86a22a8bb08e7e7bf94bdd6695b362"
        },
        "date": 1783090497364,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 214880,
            "range": "± 1712",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 399009,
            "range": "± 3997",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1266776610,
            "range": "± 749320",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 138961,
            "range": "± 2536",
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
          "id": "120ce76e84586ab349c946010d3351a1c283e2d4",
          "message": "Merge pull request #330 from haraldme/feat/log-request-path-and-method-with-error-response\n\nInclude request path+method when an error response is logged",
          "timestamp": "2026-07-07T12:13:09+02:00",
          "tree_id": "74d4973377ec6a3c8a132ca184626ba729c44700",
          "url": "https://github.com/navikt/pdfgenrs/commit/120ce76e84586ab349c946010d3351a1c283e2d4"
        },
        "date": 1783419491521,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 230808,
            "range": "± 15531",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 423194,
            "range": "± 2379",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1269956840,
            "range": "± 692442",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 155971,
            "range": "± 4190",
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
          "id": "4e5016c3bb56c52adf10a1870cd4c3b433f7497f",
          "message": "Merge pull request #331 from haraldme/feat/preserve-insert-order-in-json-log-objects\n\nUse the \"preserve_order\" feature of serde_json",
          "timestamp": "2026-07-07T12:30:09+02:00",
          "tree_id": "0be258775fc87d9c82ac9d6bd39f4983b49bd5ca",
          "url": "https://github.com/navikt/pdfgenrs/commit/4e5016c3bb56c52adf10a1870cd4c3b433f7497f"
        },
        "date": 1783420553272,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 248051,
            "range": "± 21886",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 424187,
            "range": "± 13062",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1266234577,
            "range": "± 579333",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 154750,
            "range": "± 4090",
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
          "id": "e792f97391c0ec619e1cf22b48bcd774566a6090",
          "message": "Merge pull request #332 from navikt/copilot/fix-audit-dependencies\n\nRefresh lockfile to unblock dependency audit job",
          "timestamp": "2026-07-07T13:05:37+02:00",
          "tree_id": "5a77bb8384288cd4ed8f6443dc7be6bfe2cf8392",
          "url": "https://github.com/navikt/pdfgenrs/commit/e792f97391c0ec619e1cf22b48bcd774566a6090"
        },
        "date": 1783422856942,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 231406,
            "range": "± 1805",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 428742,
            "range": "± 8798",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1270175659,
            "range": "± 858330",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 157825,
            "range": "± 3544",
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
          "id": "bfa99167e377f7e553337b3060821344128ec3ff",
          "message": "Merge pull request #334 from navikt/MikAoJk-patch-1\n\nRemove pull_request trigger from workflow",
          "timestamp": "2026-07-07T15:21:28+02:00",
          "tree_id": "2b27bf1b9be52e0c690356804a4c145fb17edc71",
          "url": "https://github.com/navikt/pdfgenrs/commit/bfa99167e377f7e553337b3060821344128ec3ff"
        },
        "date": 1783430722017,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 225057,
            "range": "± 11316",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 419140,
            "range": "± 4904",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1266268105,
            "range": "± 781471",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 143043,
            "range": "± 4378",
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
          "id": "49414304959b321fea7e88cd4a9d4e5d1949d56c",
          "message": "Merge pull request #333 from haraldme/fix/allow-request-body-limit-increase-beyond-default\n\nDisable axum's implicit DefaultBodyLimit",
          "timestamp": "2026-07-07T15:30:32+02:00",
          "tree_id": "07f8ecc814a86c3750bcb633d94c9159ae4f4618",
          "url": "https://github.com/navikt/pdfgenrs/commit/49414304959b321fea7e88cd4a9d4e5d1949d56c"
        },
        "date": 1783431267093,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 228800,
            "range": "± 13498",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 425011,
            "range": "± 6822",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1264911372,
            "range": "± 487300",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 151030,
            "range": "± 4060",
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
          "id": "cde8ad6a1a3dc425d4b8c72ba1b7d30635af0387",
          "message": "Merge pull request #335 from navikt/copilot/add-unit-test-axum-body-size-limit\n\nAdd unit tests for axum body size limit",
          "timestamp": "2026-07-07T17:37:52+02:00",
          "tree_id": "285ef5eb1ea038999508a6bdde3c826863028acd",
          "url": "https://github.com/navikt/pdfgenrs/commit/cde8ad6a1a3dc425d4b8c72ba1b7d30635af0387"
        },
        "date": 1783438927486,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 233361,
            "range": "± 18934",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 430117,
            "range": "± 3352",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1268883033,
            "range": "± 1469313",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 154535,
            "range": "± 4235",
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
          "id": "3981b516179db6f553eb9f5711132268a1f861c4",
          "message": "Merge pull request #336 from navikt/copilot/check-unit-tests-necessity\n\nAdd unit tests for low-coverage areas",
          "timestamp": "2026-07-07T18:43:18+02:00",
          "tree_id": "30f14259ca0add66d73f144b7ab7cc79e50ede46",
          "url": "https://github.com/navikt/pdfgenrs/commit/3981b516179db6f553eb9f5711132268a1f861c4"
        },
        "date": 1783442852919,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 235779,
            "range": "± 9889",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 428080,
            "range": "± 2614",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1265469152,
            "range": "± 559925",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 156909,
            "range": "± 3705",
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
          "id": "992eed8b65e7697442e40ef0073bbc3b05b448f0",
          "message": "Merge pull request #337 from navikt/copilot/add-histograms-for-compilation-duration\n\nAdd compilation duration histogram per template",
          "timestamp": "2026-07-07T20:53:37+02:00",
          "tree_id": "c86c6611b45ba0c578f276ff86c76537fc0b76d5",
          "url": "https://github.com/navikt/pdfgenrs/commit/992eed8b65e7697442e40ef0073bbc3b05b448f0"
        },
        "date": 1783450668335,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 225671,
            "range": "± 1263",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 420757,
            "range": "± 13039",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1265232094,
            "range": "± 468630",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 153675,
            "range": "± 4682",
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
          "id": "b1d6eedd1e4b4cc1615077e704b0ee13fc930ef2",
          "message": "Merge pull request #339 from navikt/copilot/graceful-semaphore-backpressure\n\nAdd Retry-After header to 503 Service Unavailable response for graceful semaphore backpressure",
          "timestamp": "2026-07-08T10:19:43+02:00",
          "tree_id": "a96f03c283ffac2707d81542355649667aa19d5f",
          "url": "https://github.com/navikt/pdfgenrs/commit/b1d6eedd1e4b4cc1615077e704b0ee13fc930ef2"
        },
        "date": 1783499035200,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 231775,
            "range": "± 12582",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 425938,
            "range": "± 3084",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 1266250263,
            "range": "± 1096022",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 158045,
            "range": "± 5756",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}