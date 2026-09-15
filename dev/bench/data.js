window.BENCHMARK_DATA = {
  "lastUpdate": 1789487290094,
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
          "id": "ac643ec3a5b7bfb18759b69dd3463f422d6da31b",
          "message": "Merge pull request #469 from hestad/fix-quick-xml-via-citationberg-patch\n\nfix(deps): patch citationberg to drop vulnerable quick-xml 0.38",
          "timestamp": "2026-09-04T18:22:46+02:00",
          "tree_id": "ae5a8c7a03fa2e34d281fcb160e85b63c74aebe6",
          "url": "https://github.com/navikt/pdfgenrs/commit/ac643ec3a5b7bfb18759b69dd3463f422d6da31b"
        },
        "date": 1788539255501,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 228333,
            "range": "± 14437",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 421890,
            "range": "± 5783",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5252202,
            "range": "± 208433",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91292,
            "range": "± 7369",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7954505,
            "range": "± 216383",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 167928,
            "range": "± 5251",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 241227,
            "range": "± 6791",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 446359,
            "range": "± 3055",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10811,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18555,
            "range": "± 260",
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
          "id": "73521968a83752a4e155f94b87b7353c9286ca7c",
          "message": "Merge pull request #470 from navikt/copilot/create-template-for-brev\n\nAdd NAV-style letter template",
          "timestamp": "2026-09-05T07:55:10+02:00",
          "tree_id": "6d4dea0bcc5d22f93450f8a0ef0bfa77ae9793a5",
          "url": "https://github.com/navikt/pdfgenrs/commit/73521968a83752a4e155f94b87b7353c9286ca7c"
        },
        "date": 1788587901780,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 211114,
            "range": "± 19878",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 405945,
            "range": "± 7767",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5421035,
            "range": "± 40832",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 85948,
            "range": "± 6320",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7390201,
            "range": "± 20391",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 156539,
            "range": "± 3129",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 214522,
            "range": "± 3901",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 442190,
            "range": "± 7929",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10941,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18195,
            "range": "± 411",
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
          "id": "4eba4d416677c216808d133a69231c99c0f5a247",
          "message": "chore: bump rust verison 1.98.1",
          "timestamp": "2026-09-07T08:24:16+02:00",
          "tree_id": "5b1a600a6feb74bb6e96305a2cdeea8301aca334",
          "url": "https://github.com/navikt/pdfgenrs/commit/4eba4d416677c216808d133a69231c99c0f5a247"
        },
        "date": 1788762469418,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 162076,
            "range": "± 1394",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 308122,
            "range": "± 3238",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4190887,
            "range": "± 48578",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 67332,
            "range": "± 4584",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6927326,
            "range": "± 472225",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 124602,
            "range": "± 1758",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 166281,
            "range": "± 3826",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 341425,
            "range": "± 1460",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 8338,
            "range": "± 299",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 14030,
            "range": "± 337",
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
          "id": "89149125995fab26ce3ead4184ac02d38db0cb7e",
          "message": "Revert \"chore: bump rust verison 1.98.1\"\n\nThis reverts commit 4eba4d416677c216808d133a69231c99c0f5a247.",
          "timestamp": "2026-09-07T10:40:47+02:00",
          "tree_id": "6d4dea0bcc5d22f93450f8a0ef0bfa77ae9793a5",
          "url": "https://github.com/navikt/pdfgenrs/commit/89149125995fab26ce3ead4184ac02d38db0cb7e"
        },
        "date": 1788770720333,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 231616,
            "range": "± 16159",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 427369,
            "range": "± 12650",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5236351,
            "range": "± 124583",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 101369,
            "range": "± 8453",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 9303171,
            "range": "± 794600",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 211473,
            "range": "± 17542",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 251554,
            "range": "± 21005",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 507056,
            "range": "± 29913",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11673,
            "range": "± 521",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18838,
            "range": "± 103",
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
          "id": "d3f752879fd36f89235ebb76da3ecb1be8a8f891",
          "message": "Merge pull request #472 from navikt/ruststable\n\nchore: bump rust version to 1.98.1",
          "timestamp": "2026-09-07T11:35:24+02:00",
          "tree_id": "a05376a817a49de127a3afb12d5ce38fcb66f353",
          "url": "https://github.com/navikt/pdfgenrs/commit/d3f752879fd36f89235ebb76da3ecb1be8a8f891"
        },
        "date": 1788773919355,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 212245,
            "range": "± 12803",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 400666,
            "range": "± 4374",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5297825,
            "range": "± 32084",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 86052,
            "range": "± 7570",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7711179,
            "range": "± 39055",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 167592,
            "range": "± 2902",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 240843,
            "range": "± 4525",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 444562,
            "range": "± 4852",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10812,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18318,
            "range": "± 376",
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
          "id": "d5f82d3d99665debb4d3857cebaf907e359028d4",
          "message": "Merge pull request #473 from navikt/dependabot/docker/clux/muslrust-bde72f6\n\nchore(deps): bump clux/muslrust from `c4058e1` to `bde72f6`",
          "timestamp": "2026-09-11T15:50:04+02:00",
          "tree_id": "64b2336d5a9cfa3c4b57e19d38326474409aefd6",
          "url": "https://github.com/navikt/pdfgenrs/commit/d5f82d3d99665debb4d3857cebaf907e359028d4"
        },
        "date": 1789134805740,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 216946,
            "range": "± 14741",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 398799,
            "range": "± 2551",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5300950,
            "range": "± 20313",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 86548,
            "range": "± 7551",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7482256,
            "range": "± 91437",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 164047,
            "range": "± 4139",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 217442,
            "range": "± 6656",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 442233,
            "range": "± 2539",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10870,
            "range": "± 124",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18182,
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
          "id": "cf9d3b74ed3b15e385186d82dc4e0cffe8794af4",
          "message": "Merge pull request #474 from navikt/dependabot/cargo/tower-http-0.7.1\n\nchore(deps): bump tower-http from 0.7.0 to 0.7.1",
          "timestamp": "2026-09-11T16:35:15+02:00",
          "tree_id": "5df08013db165aa9e81fa32b686ba10052dea33d",
          "url": "https://github.com/navikt/pdfgenrs/commit/cf9d3b74ed3b15e385186d82dc4e0cffe8794af4"
        },
        "date": 1789137521863,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 212783,
            "range": "± 11169",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 398335,
            "range": "± 2427",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5442257,
            "range": "± 86856",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 87162,
            "range": "± 7077",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 8169668,
            "range": "± 194053",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 161474,
            "range": "± 3296",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 239706,
            "range": "± 7332",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 438351,
            "range": "± 2185",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10817,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18084,
            "range": "± 53",
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
          "id": "9431e9458ca1d208062ec2ff10ad2f599e26e67b",
          "message": "Increase HTML benchmark time limits",
          "timestamp": "2026-09-15T17:16:06+02:00",
          "tree_id": "2dee4afd6e27aead3ce65fecccee8425336e6e6b",
          "url": "https://github.com/navikt/pdfgenrs/commit/9431e9458ca1d208062ec2ff10ad2f599e26e67b"
        },
        "date": 1789485561708,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 230402,
            "range": "± 26223",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 428343,
            "range": "± 3611",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5212407,
            "range": "± 38060",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91317,
            "range": "± 6526",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7910241,
            "range": "± 131821",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 175537,
            "range": "± 6513",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 248009,
            "range": "± 5920",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 458807,
            "range": "± 3381",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11129,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 19058,
            "range": "± 75",
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
          "id": "456661dcc89e462c99cbb163886351c4fd1c1c9a",
          "message": "Merge pull request #477 from navikt/copilot/pretty-print-github-actions-summary\n\nAdd dependency audit report to Actions summary",
          "timestamp": "2026-09-15T17:15:27+02:00",
          "tree_id": "1fd5a3f75c03ea1b2815bbb556513e1c04b752a0",
          "url": "https://github.com/navikt/pdfgenrs/commit/456661dcc89e462c99cbb163886351c4fd1c1c9a"
        },
        "date": 1789485569995,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 222564,
            "range": "± 3181",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 420675,
            "range": "± 24319",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5187342,
            "range": "± 41461",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91192,
            "range": "± 6482",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7948128,
            "range": "± 413284",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 162116,
            "range": "± 5792",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 236598,
            "range": "± 7669",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 444575,
            "range": "± 9268",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10744,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18445,
            "range": "± 183",
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
          "id": "a3d4510f4a87114a5a1e53e6fcacc21be10e216c",
          "message": "Merge pull request #478 from navikt/copilot/add-rustls-dependency\n\nRequire rustls 0.23.45",
          "timestamp": "2026-09-15T17:43:13+02:00",
          "tree_id": "a546e016ebcf8d03bc0d923522e547ddb5490174",
          "url": "https://github.com/navikt/pdfgenrs/commit/a3d4510f4a87114a5a1e53e6fcacc21be10e216c"
        },
        "date": 1789487277389,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 128470,
            "range": "± 3478",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 275094,
            "range": "± 9839",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 3709394,
            "range": "± 169434",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 65498,
            "range": "± 5066",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6857327,
            "range": "± 331647",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 99918,
            "range": "± 5946",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 175447,
            "range": "± 2569",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 311496,
            "range": "± 20177",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11518,
            "range": "± 551",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 17123,
            "range": "± 1168",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}