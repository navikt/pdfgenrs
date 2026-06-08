window.BENCHMARK_DATA = {
  "lastUpdate": 1780910885997,
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
      }
    ]
  }
}