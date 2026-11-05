window.BENCHMARK_DATA = {
  "lastUpdate": 1793861817638,
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
          "id": "9f1987975ec2170254b4eef5c0004af90194781f",
          "message": "Merge pull request #518 from navikt/time\n\nchore: drop the time dep",
          "timestamp": "2026-10-20T13:16:49+02:00",
          "tree_id": "210e71ae83947eae7515e76228968a2f366b51e6",
          "url": "https://github.com/navikt/pdfgenrs/commit/9f1987975ec2170254b4eef5c0004af90194781f"
        },
        "date": 1792495365293,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 162565,
            "range": "± 1182",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 310802,
            "range": "± 4468",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4262596,
            "range": "± 114848",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 66576,
            "range": "± 6233",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 9161802,
            "range": "± 397259",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 119984,
            "range": "± 1415",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 167806,
            "range": "± 8869",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 341652,
            "range": "± 1693",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 1105376,
            "range": "± 17478",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 4419651,
            "range": "± 48364",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 8587,
            "range": "± 125",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 14435,
            "range": "± 46",
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
          "id": "0786c932ec7e82da1fa752004a446a1723c3c6b7",
          "message": "Merge pull request #522 from navikt/fix\n\nchore: bump deps",
          "timestamp": "2026-10-26T09:54:26+01:00",
          "tree_id": "ff6335f557202ba44892915a937ab4124ae1dc38",
          "url": "https://github.com/navikt/pdfgenrs/commit/0786c932ec7e82da1fa752004a446a1723c3c6b7"
        },
        "date": 1793005181553,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 98070,
            "range": "± 5613",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 208656,
            "range": "± 16412",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 3411728,
            "range": "± 207805",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 53308,
            "range": "± 5664",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7960343,
            "range": "± 570122",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 77383,
            "range": "± 1248",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 114978,
            "range": "± 2399",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 232929,
            "range": "± 34630",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 826965,
            "range": "± 28680",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 3479783,
            "range": "± 186603",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 9939,
            "range": "± 123",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 14269,
            "range": "± 601",
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
          "id": "dc5ca9e81f940f4d16512ea220ab31225de23ad9",
          "message": "Merge pull request #521 from navikt/dependabot/github_actions/github/codeql-action-4.39.0\n\nchore(deps): bump github/codeql-action from 4.38.3 to 4.39.0",
          "timestamp": "2026-10-26T10:06:26+01:00",
          "tree_id": "ac3054cc37c509cfb6258d6c248359e6c0ecd634",
          "url": "https://github.com/navikt/pdfgenrs/commit/dc5ca9e81f940f4d16512ea220ab31225de23ad9"
        },
        "date": 1793005902611,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 147249,
            "range": "± 3838",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 334158,
            "range": "± 15503",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4462961,
            "range": "± 187753",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 78251,
            "range": "± 5940",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10621603,
            "range": "± 89427",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 118079,
            "range": "± 7245",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 182745,
            "range": "± 7284",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 378615,
            "range": "± 9142",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 1412191,
            "range": "± 16269",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 5165080,
            "range": "± 55798",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 12709,
            "range": "± 157",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18428,
            "range": "± 904",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joakim.taule.kartveit@nav.no",
            "name": "MikAojk",
            "username": "MikAoJk"
          },
          "committer": {
            "email": "joakim.taule.kartveit@nav.no",
            "name": "MikAojk",
            "username": "MikAoJk"
          },
          "distinct": true,
          "id": "fe7ef268bb444e437986b938383b1b513a2a64e9",
          "message": "chore: bump pulldown-cmark due to vulnerabilities",
          "timestamp": "2026-10-29T11:51:16+01:00",
          "tree_id": "819edcd48c11604b946768c2ddadd6fb863f6d6f",
          "url": "https://github.com/navikt/pdfgenrs/commit/fe7ef268bb444e437986b938383b1b513a2a64e9"
        },
        "date": 1793271369574,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 219313,
            "range": "± 4232",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 419323,
            "range": "± 20692",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5234064,
            "range": "± 16835",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 88607,
            "range": "± 7843",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 11821646,
            "range": "± 110926",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 165648,
            "range": "± 3736",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 214410,
            "range": "± 11089",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 447618,
            "range": "± 2910",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 1311919,
            "range": "± 13162",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 5066216,
            "range": "± 53904",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11176,
            "range": "± 136",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18594,
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
          "id": "891058571ebfe1054da41b8597192248933eed58",
          "message": "Merge pull request #523 from navikt/dependabot/docker/distroless/static-debian13-18f4cc2\n\nchore(deps): bump distroless/static-debian13 from `d620678` to `18f4cc2`",
          "timestamp": "2026-10-30T14:29:08+01:00",
          "tree_id": "b4df7f37ae3fbf1856f2b0ea50e3367a9c7973bf",
          "url": "https://github.com/navikt/pdfgenrs/commit/891058571ebfe1054da41b8597192248933eed58"
        },
        "date": 1793367305030,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 159392,
            "range": "± 3695",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 310499,
            "range": "± 1487",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4168212,
            "range": "± 13388",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 64608,
            "range": "± 5089",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 8890490,
            "range": "± 96692",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 112862,
            "range": "± 1330",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 161020,
            "range": "± 2794",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 335826,
            "range": "± 1483",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 1097054,
            "range": "± 6846",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 4305790,
            "range": "± 72216",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 8523,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 13933,
            "range": "± 40",
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
          "id": "0cae89f3ec8d6d1f8d7faf6fc3b684e7b871637d",
          "message": "Merge pull request #526 from navikt/chore/bump-pulldown-cmark-0.13.6\n\nchore: bump pulldown-cmark to 0.13.6",
          "timestamp": "2026-11-03T18:22:49+01:00",
          "tree_id": "358c92b9b3a358852dffe0eea78cf9365d6403d6",
          "url": "https://github.com/navikt/pdfgenrs/commit/0cae89f3ec8d6d1f8d7faf6fc3b684e7b871637d"
        },
        "date": 1793726916107,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 176333,
            "range": "± 12152",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 397630,
            "range": "± 2742",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5383327,
            "range": "± 64973",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92575,
            "range": "± 8632",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 14425686,
            "range": "± 462709",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 135571,
            "range": "± 6904",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 209341,
            "range": "± 8861",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 445330,
            "range": "± 3166",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 1647042,
            "range": "± 27256",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 6849854,
            "range": "± 330122",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 14760,
            "range": "± 106",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 21857,
            "range": "± 201",
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
          "id": "09e4bfb520674f13735eb063db3c80bda213dcc8",
          "message": "Merge pull request #525 from navikt/metrics-declared-content-length\n\nfix: clarify request content length metric",
          "timestamp": "2026-11-03T18:38:09+01:00",
          "tree_id": "e8ffb4df2dbc02d2712c48ec4caee3b54628848d",
          "url": "https://github.com/navikt/pdfgenrs/commit/09e4bfb520674f13735eb063db3c80bda213dcc8"
        },
        "date": 1793727757141,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 222722,
            "range": "± 13442",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 415922,
            "range": "± 8583",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5254929,
            "range": "± 90866",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 88685,
            "range": "± 6704",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10982240,
            "range": "± 385396",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 162906,
            "range": "± 3296",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 215425,
            "range": "± 10823",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 453091,
            "range": "± 9316",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 1334837,
            "range": "± 17298",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 5227891,
            "range": "± 145707",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11179,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18520,
            "range": "± 83",
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
          "id": "ac82780d56e261dc68d6a896dbca3f716b9893b7",
          "message": "Merge pull request #527 from navikt/error\n\nchore: added error details for typst PDF export",
          "timestamp": "2026-11-04T09:41:53+01:00",
          "tree_id": "1e3118dd864a356aed39bddce925916fbc825573",
          "url": "https://github.com/navikt/pdfgenrs/commit/ac82780d56e261dc68d6a896dbca3f716b9893b7"
        },
        "date": 1793782049775,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 166604,
            "range": "± 4970",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 384845,
            "range": "± 7174",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5096636,
            "range": "± 50004",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 87823,
            "range": "± 8726",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 12551636,
            "range": "± 205337",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 132357,
            "range": "± 5362",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 205094,
            "range": "± 8809",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 430394,
            "range": "± 8008",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 1565529,
            "range": "± 12750",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 5802126,
            "range": "± 56672",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 14536,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 20987,
            "range": "± 150",
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
          "id": "e99eb250cf1bdaff5f329fd046ace95c4d136822",
          "message": "Merge pull request #528 from navikt/copilot/clarify-response-content-length-metric\n\nClarify response content length metric",
          "timestamp": "2026-11-04T19:05:45+01:00",
          "tree_id": "5c9aaaeaded871f88368e6bdbcade2c81f7e71ba",
          "url": "https://github.com/navikt/pdfgenrs/commit/e99eb250cf1bdaff5f329fd046ace95c4d136822"
        },
        "date": 1793815825607,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 220827,
            "range": "± 10868",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 417505,
            "range": "± 2649",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5314833,
            "range": "± 100983",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 87612,
            "range": "± 7768",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 11685974,
            "range": "± 129407",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 164927,
            "range": "± 4141",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 214772,
            "range": "± 7043",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 451314,
            "range": "± 1854",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 1310343,
            "range": "± 13243",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 5106030,
            "range": "± 66112",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11107,
            "range": "± 420",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18729,
            "range": "± 243",
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
          "id": "9abb77c769fdfe8eeb3fb8923c2b30c77bf9ab4a",
          "message": "Merge pull request #529 from navikt/copilot/rust-book-cleanups\n\nReduce redundant Rust result and route clones",
          "timestamp": "2026-11-05T07:51:46+01:00",
          "tree_id": "a9f518f69d5b13acb18b47393a56ff77c369187b",
          "url": "https://github.com/navikt/pdfgenrs/commit/9abb77c769fdfe8eeb3fb8923c2b30c77bf9ab4a"
        },
        "date": 1793861805389,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 138894,
            "range": "± 3744",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 310717,
            "range": "± 6222",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4148432,
            "range": "± 86877",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 68936,
            "range": "± 8047",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 13858172,
            "range": "± 1898278",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 107713,
            "range": "± 5242",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 174230,
            "range": "± 4872",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 354976,
            "range": "± 7925",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 1357683,
            "range": "± 33885",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 5241738,
            "range": "± 67909",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11269,
            "range": "± 356",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 16559,
            "range": "± 465",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}