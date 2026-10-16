window.BENCHMARK_DATA = {
  "lastUpdate": 1792160863610,
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
          "id": "7e88f2b45e762788691f0ea821d5d1128e064583",
          "message": "Merge pull request #510 from navikt/copilot/expand-performance-coverage\n\nExpand benchmarks for large images and saturated mixed workloads",
          "timestamp": "2026-10-09T14:08:21+02:00",
          "tree_id": "667c480ce257b46ea4e68cb499faad41cb401380",
          "url": "https://github.com/navikt/pdfgenrs/commit/7e88f2b45e762788691f0ea821d5d1128e064583"
        },
        "date": 1791547907808,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 228207,
            "range": "± 13161",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 425015,
            "range": "± 12671",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5242600,
            "range": "± 57632",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91401,
            "range": "± 6966",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10339479,
            "range": "± 40401",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 166627,
            "range": "± 5578",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 219107,
            "range": "± 7458",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 456816,
            "range": "± 1480",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 1292033,
            "range": "± 7148",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 4798842,
            "range": "± 26934",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10932,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18962,
            "range": "± 110",
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
          "id": "e433d2e8d55edf5fbc0b2fd3c851b2db001e04bf",
          "message": "Merge pull request #511 from navikt/dependabot/github_actions/release-drafter/release-drafter-7.9.0\n\nchore(deps): bump release-drafter/release-drafter from 7.7.0 to 7.9.0",
          "timestamp": "2026-10-09T15:55:03+02:00",
          "tree_id": "75557093975e6135854bc28633dcf871d9d96b8f",
          "url": "https://github.com/navikt/pdfgenrs/commit/e433d2e8d55edf5fbc0b2fd3c851b2db001e04bf"
        },
        "date": 1791554304130,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 228608,
            "range": "± 1700",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 427241,
            "range": "± 3381",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5408028,
            "range": "± 112145",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91143,
            "range": "± 7080",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10698369,
            "range": "± 154160",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 168404,
            "range": "± 5201",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 221255,
            "range": "± 9148",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 457893,
            "range": "± 3976",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 1304320,
            "range": "± 19709",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 4887125,
            "range": "± 81581",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10888,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 19071,
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
          "id": "48ee0a1fef17fedba46c64e9d763648043901ed9",
          "message": "Merge pull request #513 from navikt/clap\n\nRead configuration using clap",
          "timestamp": "2026-10-16T10:20:30+02:00",
          "tree_id": "b3be9a1c047a905826e3c19c919896d23e4da9e1",
          "url": "https://github.com/navikt/pdfgenrs/commit/48ee0a1fef17fedba46c64e9d763648043901ed9"
        },
        "date": 1792139467959,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 216297,
            "range": "± 13357",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 400617,
            "range": "± 2145",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5365760,
            "range": "± 43271",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 86773,
            "range": "± 5354",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 11101607,
            "range": "± 59749",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 158395,
            "range": "± 2073",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 218144,
            "range": "± 2406",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 442393,
            "range": "± 2712",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 1404220,
            "range": "± 8076",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 5459726,
            "range": "± 18244",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10809,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18350,
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
          "id": "71b0b8e564eb4b33ef78ab0f7e0c9f73a60f6d30",
          "message": "Merge pull request #514 from navikt/bump\n\nchore: bump some deps",
          "timestamp": "2026-10-16T14:27:35+02:00",
          "tree_id": "00204db6fb855f99e690816b460586670f068b09",
          "url": "https://github.com/navikt/pdfgenrs/commit/71b0b8e564eb4b33ef78ab0f7e0c9f73a60f6d30"
        },
        "date": 1792153892174,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 231161,
            "range": "± 13102",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 424292,
            "range": "± 3039",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5335510,
            "range": "± 81416",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91406,
            "range": "± 9234",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 10698599,
            "range": "± 139098",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 165102,
            "range": "± 4320",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 220285,
            "range": "± 8570",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 458319,
            "range": "± 4731",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 1289182,
            "range": "± 17006",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 5061693,
            "range": "± 70687",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11003,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18885,
            "range": "± 100",
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
          "id": "134df5886b2282f2939b10b67c83dc8fa8c37c6d",
          "message": "Merge pull request #516 from navikt/dependabot/github_actions/github/codeql-action-4.38.3\n\nchore(deps): bump github/codeql-action from 4.38.2 to 4.38.3",
          "timestamp": "2026-10-16T16:23:25+02:00",
          "tree_id": "28732148ec43fd7e945cd6ed31d1865f4d2a098b",
          "url": "https://github.com/navikt/pdfgenrs/commit/134df5886b2282f2939b10b67c83dc8fa8c37c6d"
        },
        "date": 1792160853242,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 163992,
            "range": "± 9142",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 310408,
            "range": "± 1663",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4157289,
            "range": "± 17908",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 66990,
            "range": "± 5236",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 8801599,
            "range": "± 130429",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 119373,
            "range": "± 4475",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 165441,
            "range": "± 5119",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 339420,
            "range": "± 1406",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/2048",
            "value": 1086822,
            "range": "± 4687",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_large_raster/png/4096",
            "value": 4337129,
            "range": "± 37361",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 8406,
            "range": "± 261",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 14104,
            "range": "± 185",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}