window.BENCHMARK_DATA = {
  "lastUpdate": 1787319635782,
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
          "id": "e4031c9d15aa6596183e176861599d11a6f9e1f4",
          "message": "chore: small fixes",
          "timestamp": "2026-08-13T13:37:13+02:00",
          "tree_id": "14b9bbde8f8d81949ddbe8c859e79aba9d4cfa1a",
          "url": "https://github.com/navikt/pdfgenrs/commit/e4031c9d15aa6596183e176861599d11a6f9e1f4"
        },
        "date": 1786621194658,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 218673,
            "range": "± 13996",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 402764,
            "range": "± 6276",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5247444,
            "range": "± 94595",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 87488,
            "range": "± 6043",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 150359,
            "range": "± 2922",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 215527,
            "range": "± 6177",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 357802,
            "range": "± 2176",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10910,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18366,
            "range": "± 71",
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
          "id": "b97611bf0a747c5115d821788089a2c7dbfdecd9",
          "message": "Merge pull request #422 from navikt/dependabot/cargo/time-0.3.55\n\nchore(deps): bump time from 0.3.54 to 0.3.55",
          "timestamp": "2026-08-14T16:51:03+02:00",
          "tree_id": "2b9a2385154da1b597ba3a9f5baad419cdfe4d13",
          "url": "https://github.com/navikt/pdfgenrs/commit/b97611bf0a747c5115d821788089a2c7dbfdecd9"
        },
        "date": 1786719340215,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 231499,
            "range": "± 11915",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 432068,
            "range": "± 8170",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5202836,
            "range": "± 102080",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92213,
            "range": "± 6884",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 161321,
            "range": "± 4537",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 215114,
            "range": "± 10626",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 367394,
            "range": "± 2464",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10979,
            "range": "± 506",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18868,
            "range": "± 269",
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
          "id": "44609a4073e78d9ae49fb1f0b40396bf2c1fa184",
          "message": "Merge pull request #423 from navikt/dependabot/github_actions/github/codeql-action-4.37.6\n\nchore(deps): bump github/codeql-action from 4.37.4 to 4.37.6",
          "timestamp": "2026-08-14T16:51:23+02:00",
          "tree_id": "07facc01f95092c4da4bd2d3da8dcf898581cb2e",
          "url": "https://github.com/navikt/pdfgenrs/commit/44609a4073e78d9ae49fb1f0b40396bf2c1fa184"
        },
        "date": 1786719356840,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 231397,
            "range": "± 14835",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 427361,
            "range": "± 2381",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5172280,
            "range": "± 75081",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92317,
            "range": "± 6518",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 157269,
            "range": "± 4298",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 211917,
            "range": "± 6928",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 360811,
            "range": "± 1586",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10849,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18673,
            "range": "± 61",
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
          "id": "c5ffb0ac565ef87d9e159bbcc527a9204b86ad2b",
          "message": "Merge pull request #424 from navikt/copilot/restore-and-update-pr-365\n\nHTML-to-PDF endpoint and bump ironpress to 1.5.2",
          "timestamp": "2026-08-20T13:03:52+02:00",
          "tree_id": "0ec1114e08990e0789a6102fe3b62e5fbabff808",
          "url": "https://github.com/navikt/pdfgenrs/commit/c5ffb0ac565ef87d9e159bbcc527a9204b86ad2b"
        },
        "date": 1787224185773,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 234932,
            "range": "± 2641",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 424938,
            "range": "± 2569",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5292471,
            "range": "± 61594",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92808,
            "range": "± 6541",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6163160,
            "range": "± 45931",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 158482,
            "range": "± 4730",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 223381,
            "range": "± 9157",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 364048,
            "range": "± 2245",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11224,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18973,
            "range": "± 132",
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
          "id": "5a27f3dc432c949e2f0e3ad08d1303203afe5e37",
          "message": "Merge pull request #427 from navikt/dependabot/cargo/http-body-util-0.1.5\n\nchore(deps): bump http-body-util from 0.1.4 to 0.1.5",
          "timestamp": "2026-08-21T15:29:30+02:00",
          "tree_id": "0bb1b1d1cce4cb40ec590b4045fc8f66b7790e4c",
          "url": "https://github.com/navikt/pdfgenrs/commit/5a27f3dc432c949e2f0e3ad08d1303203afe5e37"
        },
        "date": 1787319632244,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 215685,
            "range": "± 6769",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 400254,
            "range": "± 1926",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5418732,
            "range": "± 260844",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 87664,
            "range": "± 5686",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6546254,
            "range": "± 122407",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 151189,
            "range": "± 2879",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 212300,
            "range": "± 3925",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 358775,
            "range": "± 2438",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10949,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18508,
            "range": "± 165",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}