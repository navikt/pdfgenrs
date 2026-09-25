window.BENCHMARK_DATA = {
  "lastUpdate": 1790323895782,
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
          "id": "b93ce579ddc56c7b5702ee9fd6d3bb89e745d482",
          "message": "Merge pull request #483 from navikt/panic\n\nchore: avoid panics due to max_concurrent_compilations is above Semaphore::MAX_PERMITS",
          "timestamp": "2026-09-17T12:57:45+02:00",
          "tree_id": "6bc34ef40359a0a72261f69d2ac7ed43028cd107",
          "url": "https://github.com/navikt/pdfgenrs/commit/b93ce579ddc56c7b5702ee9fd6d3bb89e745d482"
        },
        "date": 1789642866280,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 199969,
            "range": "± 15547",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 402822,
            "range": "± 52267",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5499554,
            "range": "± 51034",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 93523,
            "range": "± 7943",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10993017,
            "range": "± 80311",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 146135,
            "range": "± 7267",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 239241,
            "range": "± 11050",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 442488,
            "range": "± 30254",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 14301,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 21452,
            "range": "± 358",
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
          "id": "35a6acf5304f1394b069d9191afa9b9e64dbef2d",
          "message": "Merge pull request #485 from navikt/dependabot/github_actions/github/codeql-action-4.38.0\n\nchore(deps): bump github/codeql-action from 4.37.9 to 4.38.0",
          "timestamp": "2026-09-18T17:03:23+02:00",
          "tree_id": "07ad89543ceda0bed39868b84236c61a3b209597",
          "url": "https://github.com/navikt/pdfgenrs/commit/35a6acf5304f1394b069d9191afa9b9e64dbef2d"
        },
        "date": 1789743997920,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 223938,
            "range": "± 32573",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 417125,
            "range": "± 4549",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5236855,
            "range": "± 180572",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 90775,
            "range": "± 7028",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7738902,
            "range": "± 211002",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 167896,
            "range": "± 6063",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 235531,
            "range": "± 5409",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 443107,
            "range": "± 6561",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10801,
            "range": "± 459",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18704,
            "range": "± 298",
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
          "id": "a7bfebb1de1c1fd2428d0662304ddd08624fe1f8",
          "message": "Merge pull request #484 from navikt/dependabot/docker/distroless/static-debian13-e2e927e\n\nchore(deps): bump distroless/static-debian13 from `1c2c046` to `e2e927e`",
          "timestamp": "2026-09-18T17:03:37+02:00",
          "tree_id": "a5023dcfd53ad2f59af8e93ad500e33b2000fe4b",
          "url": "https://github.com/navikt/pdfgenrs/commit/a7bfebb1de1c1fd2428d0662304ddd08624fe1f8"
        },
        "date": 1789744043732,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 128050,
            "range": "± 12086",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 277572,
            "range": "± 7484",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 3693454,
            "range": "± 266821",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 64611,
            "range": "± 6015",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6626412,
            "range": "± 232793",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 100458,
            "range": "± 5649",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 175895,
            "range": "± 8856",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 309553,
            "range": "± 17714",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 9823,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 14866,
            "range": "± 795",
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
          "id": "9ca6fd9b9c05502f50fff4731ed800d009c77ad8",
          "message": "Merge pull request #486 from navikt/copilot/generic-parser-for-numeric-parsing\n\nRefactor numeric configuration parsing",
          "timestamp": "2026-09-19T13:28:04+02:00",
          "tree_id": "871d48745c858340c0f523b581005dc7ed60c8c5",
          "url": "https://github.com/navikt/pdfgenrs/commit/9ca6fd9b9c05502f50fff4731ed800d009c77ad8"
        },
        "date": 1789817491484,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 214840,
            "range": "± 7425",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 404066,
            "range": "± 5439",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5360111,
            "range": "± 46099",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 87461,
            "range": "± 5469",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7741621,
            "range": "± 254259",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 166385,
            "range": "± 3171",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 242546,
            "range": "± 5046",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 441578,
            "range": "± 2345",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10932,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18329,
            "range": "± 121",
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
          "id": "24d69b3b38f7c3f5709e4d5b7c155bc5211c2fd3",
          "message": "Merge pull request #487 from navikt/copilot/validate-clamp-max-concurrent-compilations\n\nValidate semaphore permit configuration",
          "timestamp": "2026-09-19T13:30:57+02:00",
          "tree_id": "6aae74ec1e9051833d3be98f236f442c0fe0e37b",
          "url": "https://github.com/navikt/pdfgenrs/commit/24d69b3b38f7c3f5709e4d5b7c155bc5211c2fd3"
        },
        "date": 1789817660377,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 150569,
            "range": "± 8016",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 330531,
            "range": "± 3379",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4377643,
            "range": "± 26472",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 79123,
            "range": "± 6904",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6771522,
            "range": "± 42534",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 117042,
            "range": "± 4933",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 168133,
            "range": "± 6667",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 366893,
            "range": "± 4660",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 12125,
            "range": "± 147",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18284,
            "range": "± 25",
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
          "id": "fccbdce5852dcb1ed8ef98cdcf716e79991c0699",
          "message": "Merge pull request #488 from navikt/copilot/review-testutil-module\n\nHide test helpers from the public library API",
          "timestamp": "2026-09-20T16:43:27+02:00",
          "tree_id": "8fa79a3808ed1801c6fb5f5807143ebb9db55647",
          "url": "https://github.com/navikt/pdfgenrs/commit/fccbdce5852dcb1ed8ef98cdcf716e79991c0699"
        },
        "date": 1789915608817,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 125265,
            "range": "± 3989",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 277640,
            "range": "± 13687",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 3684950,
            "range": "± 148405",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 64418,
            "range": "± 5347",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6574555,
            "range": "± 267363",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 97856,
            "range": "± 5705",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 173865,
            "range": "± 9797",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 307738,
            "range": "± 7243",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 9831,
            "range": "± 242",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 15095,
            "range": "± 854",
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
          "id": "4ca4d818b6abc25288ff018cd16d7f12a0d5f4b0",
          "message": "Merge pull request #489 from navikt/copilot/add-startup-validation-image-limit\n\nWarn on conflicting image limits",
          "timestamp": "2026-09-21T07:18:40+02:00",
          "tree_id": "b318184d870e3bca264520676d26cb69089274c9",
          "url": "https://github.com/navikt/pdfgenrs/commit/4ca4d818b6abc25288ff018cd16d7f12a0d5f4b0"
        },
        "date": 1789968127549,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 164373,
            "range": "± 8222",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 311855,
            "range": "± 1343",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4172197,
            "range": "± 26084",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 67211,
            "range": "± 6678",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6401539,
            "range": "± 73936",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 128586,
            "range": "± 2622",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 168671,
            "range": "± 5269",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 337187,
            "range": "± 1329",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 8405,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 13964,
            "range": "± 42",
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
          "id": "f75ae2435b5f01118309bf8146a27aa69cf3b17f",
          "message": "Merge pull request #491 from navikt/bumpdeps\n\nchore: bump some deps",
          "timestamp": "2026-09-24T13:12:20+02:00",
          "tree_id": "81975baa3baef872b4380768da3bac70c0c89b39",
          "url": "https://github.com/navikt/pdfgenrs/commit/f75ae2435b5f01118309bf8146a27aa69cf3b17f"
        },
        "date": 1790248561850,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 189389,
            "range": "± 8447",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 397411,
            "range": "± 2011",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5385883,
            "range": "± 74545",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91811,
            "range": "± 6852",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 11194119,
            "range": "± 242025",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 146698,
            "range": "± 7305",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 210345,
            "range": "± 5572",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 428644,
            "range": "± 6340",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 14386,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 21469,
            "range": "± 235",
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
          "id": "877badae2e4a6d23f8dca58d360f4e25cb9258e4",
          "message": "Merge pull request #492 from navikt/copilot/public-facing-modules-typed-errors\n\nReplace public `anyhow::Result` in exported APIs with typed error enums",
          "timestamp": "2026-09-24T18:51:34+02:00",
          "tree_id": "0371828571a9ebda6ab201a72ca6e130e2f9911f",
          "url": "https://github.com/navikt/pdfgenrs/commit/877badae2e4a6d23f8dca58d360f4e25cb9258e4"
        },
        "date": 1790268911717,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 196687,
            "range": "± 12590",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 354538,
            "range": "± 19615",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 6012619,
            "range": "± 423087",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 81934,
            "range": "± 8951",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 9954901,
            "range": "± 576618",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 149158,
            "range": "± 9098",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 197823,
            "range": "± 13296",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 345254,
            "range": "± 13015",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 9095,
            "range": "± 671",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 14278,
            "range": "± 421",
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
          "id": "ac1b0fae97db202979dec4863f1b5063476ae993",
          "message": "Merge pull request #493 from navikt/concurrency\n\nchore: added concurrency cancellation for workflows",
          "timestamp": "2026-09-25T10:07:53+02:00",
          "tree_id": "c925d2b0a440ae213c55f49aec581ef377020510",
          "url": "https://github.com/navikt/pdfgenrs/commit/ac1b0fae97db202979dec4863f1b5063476ae993"
        },
        "date": 1790323887909,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 164138,
            "range": "± 1085",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 310621,
            "range": "± 1765",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4163967,
            "range": "± 31095",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 67291,
            "range": "± 3775",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6179028,
            "range": "± 94633",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 124710,
            "range": "± 2575",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 168219,
            "range": "± 3586",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 336885,
            "range": "± 7897",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 8391,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 14113,
            "range": "± 68",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}