window.BENCHMARK_DATA = {
  "lastUpdate": 1790947627657,
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
          "id": "3518f5f3e4ca751e107f5bda185ea69efb142db9",
          "message": "Merge pull request #494 from navikt/dependabot/cargo/ironpress-1.7.0\n\nchore(deps): bump ironpress from 1.6.0 to 1.7.0",
          "timestamp": "2026-09-25T15:33:46+02:00",
          "tree_id": "3bd2bca502c72bff9e2bd695ee4d0bc4ec8b7993",
          "url": "https://github.com/navikt/pdfgenrs/commit/3518f5f3e4ca751e107f5bda185ea69efb142db9"
        },
        "date": 1790343723617,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 210637,
            "range": "± 4150",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 399293,
            "range": "± 1710",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5338942,
            "range": "± 42290",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 86718,
            "range": "± 6798",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10985867,
            "range": "± 25678",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 148727,
            "range": "± 3823",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 214184,
            "range": "± 4622",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 436349,
            "range": "± 11683",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10866,
            "range": "± 237",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18195,
            "range": "± 55",
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
          "id": "06248fdde747cd3f81627f040c21dc3f4c1bee57",
          "message": "Merge pull request #496 from navikt/copilot/pin-benchmark-workflow-to-1-98-1\n\nPin benchmark workflow to Rust 1.98.1",
          "timestamp": "2026-09-26T17:09:16+02:00",
          "tree_id": "87bead97329003563a6a6805a9525492dc8fdd15",
          "url": "https://github.com/navikt/pdfgenrs/commit/06248fdde747cd3f81627f040c21dc3f4c1bee57"
        },
        "date": 1790435592810,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 228244,
            "range": "± 2692",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 424099,
            "range": "± 9643",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5288903,
            "range": "± 60158",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91758,
            "range": "± 7902",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10432605,
            "range": "± 134449",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 177955,
            "range": "± 4374",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 222716,
            "range": "± 9147",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 451406,
            "range": "± 2688",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10991,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18864,
            "range": "± 273",
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
          "id": "32e3a5570d0e83fc9d5151a6d3c44bd6d9d85782",
          "message": "Merge pull request #500 from navikt/copilot/improve-performance\n\nReuse prevalidated image dimensions in image PDF generation path",
          "timestamp": "2026-09-27T08:56:07+02:00",
          "tree_id": "cc4253137033536ee86bd122d9f83d709b8b33ea",
          "url": "https://github.com/navikt/pdfgenrs/commit/32e3a5570d0e83fc9d5151a6d3c44bd6d9d85782"
        },
        "date": 1790492803143,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 232852,
            "range": "± 16077",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 425660,
            "range": "± 13774",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5318839,
            "range": "± 116949",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91432,
            "range": "± 8464",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10358882,
            "range": "± 134179",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 174340,
            "range": "± 5448",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 223962,
            "range": "± 4772",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 452208,
            "range": "± 2403",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10924,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18910,
            "range": "± 120",
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
          "id": "67943e4be6a261654ccc53de0027bf71824c7680",
          "message": "Merge pull request #502 from navikt/copilot/add-focused-tests-for-coverage\n\nAdd request-id middleware tests for conflicting response headers",
          "timestamp": "2026-09-27T14:34:02+02:00",
          "tree_id": "92b204fd6f0096540954483c7c7bfae69adce2e4",
          "url": "https://github.com/navikt/pdfgenrs/commit/67943e4be6a261654ccc53de0027bf71824c7680"
        },
        "date": 1790512609946,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 228984,
            "range": "± 15811",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 422073,
            "range": "± 5398",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5360530,
            "range": "± 52102",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91256,
            "range": "± 7799",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10560242,
            "range": "± 119749",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 168874,
            "range": "± 5811",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 216045,
            "range": "± 5069",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 451120,
            "range": "± 1667",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11009,
            "range": "± 132",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18984,
            "range": "± 165",
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
          "id": "93bf4c380a6e6918c7532c363deb4bafee7ba817",
          "message": "Merge pull request #505 from tidnav/runtime-metrics\n\nAdd runtime, process, and Tokio metrics",
          "timestamp": "2026-09-29T09:39:28+02:00",
          "tree_id": "ac25b05869cec846b3310631b40c3a6e7433374c",
          "url": "https://github.com/navikt/pdfgenrs/commit/93bf4c380a6e6918c7532c363deb4bafee7ba817"
        },
        "date": 1790667752784,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 165099,
            "range": "± 8208",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 368617,
            "range": "± 11444",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4974047,
            "range": "± 66316",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91278,
            "range": "± 5632",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 11760130,
            "range": "± 233999",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 129640,
            "range": "± 8170",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 207092,
            "range": "± 11106",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 426740,
            "range": "± 7513",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 14018,
            "range": "± 193",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 20796,
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
          "id": "6f34efbf850115aede17074be872777f6c8cfbc5",
          "message": "Merge pull request #506 from navikt/copilot/add-integration-tests\n\nAdd integration coverage for API errors and shutdown",
          "timestamp": "2026-09-29T17:08:50+02:00",
          "tree_id": "dd75b277869c31ce02e8b57e8f966a07a748f744",
          "url": "https://github.com/navikt/pdfgenrs/commit/6f34efbf850115aede17074be872777f6c8cfbc5"
        },
        "date": 1790694704415,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 165143,
            "range": "± 3089",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 311686,
            "range": "± 7176",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4126664,
            "range": "± 12902",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 67563,
            "range": "± 5493",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 8793452,
            "range": "± 22088",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 125859,
            "range": "± 1778",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 167735,
            "range": "± 5414",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 341361,
            "range": "± 7674",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 8453,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 14088,
            "range": "± 31",
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
          "id": "884f9150274648a452da61e4d6128288d6498b58",
          "message": "Merge pull request #507 from navikt/MikAoJk-patch-1\n\nchore: update ironpress version to 1.7.0",
          "timestamp": "2026-09-30T07:33:17+02:00",
          "tree_id": "7a4305e83ea38003e20e2232e98c9a7074d74d35",
          "url": "https://github.com/navikt/pdfgenrs/commit/884f9150274648a452da61e4d6128288d6498b58"
        },
        "date": 1790746570317,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 241958,
            "range": "± 19712",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 414038,
            "range": "± 20291",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5251428,
            "range": "± 83387",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 89238,
            "range": "± 6313",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10520818,
            "range": "± 70234",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 157265,
            "range": "± 4639",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 216537,
            "range": "± 7644",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 440514,
            "range": "± 1971",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10895,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18871,
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
          "id": "7618d2e7b657306c30a94538fed1966d573d53f9",
          "message": "Merge pull request #508 from tidnav/image-crop-bug\n\nfix(rendering): stop Typst's default image fit from cropping uploads",
          "timestamp": "2026-10-01T11:27:15+02:00",
          "tree_id": "04bb1b7ab878c48b10cd849a211aade33d2878e8",
          "url": "https://github.com/navikt/pdfgenrs/commit/7618d2e7b657306c30a94538fed1966d573d53f9"
        },
        "date": 1790847009771,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 96967,
            "range": "± 3552",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 204861,
            "range": "± 3577",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 3266287,
            "range": "± 84384",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 48718,
            "range": "± 3317",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7468389,
            "range": "± 119628",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 81663,
            "range": "± 3706",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 116850,
            "range": "± 3240",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 224932,
            "range": "± 16348",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 9503,
            "range": "± 183",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 14009,
            "range": "± 679",
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
          "id": "3027fdbec54f228ea75a63fe887856f3901e8482",
          "message": "Merge pull request #509 from navikt/dependabot/github_actions/github/codeql-action-4.38.2\n\nchore(deps): bump github/codeql-action from 4.38.1 to 4.38.2",
          "timestamp": "2026-10-02T15:23:58+02:00",
          "tree_id": "4425bb842c4103c69a7c0a4afd5b9ca70df349d9",
          "url": "https://github.com/navikt/pdfgenrs/commit/3027fdbec54f228ea75a63fe887856f3901e8482"
        },
        "date": 1790947618733,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 227000,
            "range": "± 13564",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 426070,
            "range": "± 10875",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5310391,
            "range": "± 121337",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91128,
            "range": "± 6826",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10580189,
            "range": "± 154271",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 165195,
            "range": "± 5610",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 229940,
            "range": "± 10641",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 458188,
            "range": "± 3627",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10596,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18458,
            "range": "± 83",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}