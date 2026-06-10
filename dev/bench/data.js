window.BENCHMARK_DATA = {
  "lastUpdate": 1781094568125,
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
      }
    ]
  }
}