window.BENCHMARK_DATA = {
  "lastUpdate": 1780914401145,
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
      }
    ]
  }
}