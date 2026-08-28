window.BENCHMARK_DATA = {
  "lastUpdate": 1787913557638,
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
          "id": "42d4343ceeb97dbae5fb5588f5b51eeddad86d84",
          "message": "Merge pull request #428 from navikt/dependabot/github_actions/github/codeql-action-4.37.7\n\nchore(deps): bump github/codeql-action from 4.37.6 to 4.37.7",
          "timestamp": "2026-08-21T15:30:43+02:00",
          "tree_id": "eabbe87305eb3e7de7be66152be892d4c3f5ff6f",
          "url": "https://github.com/navikt/pdfgenrs/commit/42d4343ceeb97dbae5fb5588f5b51eeddad86d84"
        },
        "date": 1787319645281,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 153004,
            "range": "± 3257",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 348502,
            "range": "± 10240",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4458093,
            "range": "± 70901",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 85030,
            "range": "± 7152",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 5893396,
            "range": "± 246487",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 113338,
            "range": "± 7215",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 176186,
            "range": "± 9459",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 289624,
            "range": "± 4305",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 12476,
            "range": "± 134",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18954,
            "range": "± 465",
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
          "id": "845a44710fcc54867eddaad466863e5e78a14ecb",
          "message": "Merge pull request #429 from navikt/copilot/add-integration-tests\n\nAdd integration coverage for shutdown, asset-dir failures, symlink escape paths, and 413 metrics",
          "timestamp": "2026-08-24T09:06:51+02:00",
          "tree_id": "8a18e378286e84df13a310e62e66a7fb935ef0e6",
          "url": "https://github.com/navikt/pdfgenrs/commit/845a44710fcc54867eddaad466863e5e78a14ecb"
        },
        "date": 1787555397118,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 155761,
            "range": "± 7663",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 333927,
            "range": "± 8750",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4332885,
            "range": "± 25429",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 81853,
            "range": "± 6275",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 5744254,
            "range": "± 25933",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 113012,
            "range": "± 5957",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 175882,
            "range": "± 8663",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 273778,
            "range": "± 5165",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 12186,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18439,
            "range": "± 43",
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
          "id": "b3f0f33cf44b398a9a20e54fd92d9742cd69a3cb",
          "message": "Merge pull request #430 from navikt/copilot/add-gauges-counters\n\nAdd compilation capacity metrics",
          "timestamp": "2026-08-26T08:49:21+02:00",
          "tree_id": "70b16d3690ba887b57423b455bcae909f3837540",
          "url": "https://github.com/navikt/pdfgenrs/commit/b3f0f33cf44b398a9a20e54fd92d9742cd69a3cb"
        },
        "date": 1787727142740,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 216250,
            "range": "± 13064",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 398712,
            "range": "± 12016",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5198796,
            "range": "± 28432",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 87672,
            "range": "± 8223",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6205376,
            "range": "± 310846",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 152861,
            "range": "± 2244",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 209200,
            "range": "± 2673",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 356851,
            "range": "± 7671",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10911,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18239,
            "range": "± 56",
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
          "id": "0b73d4bdcbc3e82b8cec4946aecc80d32fe42e17",
          "message": "Merge pull request #433 from navikt/copilot/split-codebase-into-folders\n\nRefactor source layout into rendering and HTTP modules",
          "timestamp": "2026-08-26T11:38:03+02:00",
          "tree_id": "10b102dd8cc37b0d82ded9a24e81de19e109ee53",
          "url": "https://github.com/navikt/pdfgenrs/commit/0b73d4bdcbc3e82b8cec4946aecc80d32fe42e17"
        },
        "date": 1787737266116,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 216023,
            "range": "± 8672",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 399027,
            "range": "± 13437",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5149044,
            "range": "± 28685",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 89248,
            "range": "± 6344",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6621189,
            "range": "± 185923",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 156251,
            "range": "± 5171",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 213587,
            "range": "± 7271",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 353636,
            "range": "± 8435",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10862,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18270,
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
          "id": "ae178e87893989c7f3a6896da9d69d8eb33178a3",
          "message": "Merge pull request #434 from navikt/arabic\n\nchore: Add Noto Sans Arabic font",
          "timestamp": "2026-08-26T11:50:07+02:00",
          "tree_id": "e811096735e50605b93ee64522d52cde95d37239",
          "url": "https://github.com/navikt/pdfgenrs/commit/ae178e87893989c7f3a6896da9d69d8eb33178a3"
        },
        "date": 1787737999936,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 230467,
            "range": "± 11495",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 427612,
            "range": "± 16183",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5255410,
            "range": "± 40297",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92603,
            "range": "± 8277",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6337680,
            "range": "± 76640",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 157928,
            "range": "± 4039",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 217303,
            "range": "± 10444",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 448687,
            "range": "± 3478",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11067,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18794,
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
          "id": "bd2e8be21ce17527fa01446b7baec67fd1df77b3",
          "message": "Merge pull request #435 from navikt/copilot/replace-global-font-cache\n\nCache HTML fonts per directory",
          "timestamp": "2026-08-26T12:26:03+02:00",
          "tree_id": "84ff2bf65af0dea518e9960d86f8f3edb79f36f9",
          "url": "https://github.com/navikt/pdfgenrs/commit/bd2e8be21ce17527fa01446b7baec67fd1df77b3"
        },
        "date": 1787740139320,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 227653,
            "range": "± 5553",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 427684,
            "range": "± 4348",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5480737,
            "range": "± 135503",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91182,
            "range": "± 7699",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6461026,
            "range": "± 90612",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 162134,
            "range": "± 5485",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 213552,
            "range": "± 9713",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 445987,
            "range": "± 2954",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10813,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18665,
            "range": "± 143",
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
          "id": "81721e2e4d5ad055d3f3e5bd39e36f9ff84cc164",
          "message": "chore: replace Match expression with a method call",
          "timestamp": "2026-08-26T14:16:02+02:00",
          "tree_id": "006a8e1b6b28315f97f8035bef8f647ba13a471c",
          "url": "https://github.com/navikt/pdfgenrs/commit/81721e2e4d5ad055d3f3e5bd39e36f9ff84cc164"
        },
        "date": 1787746751141,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 188727,
            "range": "± 14428",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 405111,
            "range": "± 17421",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5415977,
            "range": "± 91619",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92628,
            "range": "± 5056",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 8950220,
            "range": "± 101496",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 139163,
            "range": "± 5251",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 213436,
            "range": "± 9601",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 429314,
            "range": "± 14637",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 14201,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 20974,
            "range": "± 56",
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
          "id": "07c67b18e14c835a7af8160f145a5bf0957004ef",
          "message": "Merge pull request #436 from navikt/copilot/move-const-to-environment-variables\n\nConfigure image upload limits",
          "timestamp": "2026-08-27T10:21:17+02:00",
          "tree_id": "f60052a1a399ec34cae4b968fe264168b49e613a",
          "url": "https://github.com/navikt/pdfgenrs/commit/07c67b18e14c835a7af8160f145a5bf0957004ef"
        },
        "date": 1787819051887,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 213376,
            "range": "± 2037",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 399164,
            "range": "± 15262",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5327489,
            "range": "± 85099",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 86704,
            "range": "± 4100",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6299703,
            "range": "± 108150",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 151983,
            "range": "± 3465",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 212429,
            "range": "± 5019",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 429631,
            "range": "± 1935",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10788,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18072,
            "range": "± 65",
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
          "id": "a55ee2e0acfdf5d4d5ec8b49fac0f671fb44b8fd",
          "message": "Merge pull request #438 from tidnav/fix/image-cropping\n\nAvoid cropping images when converting to pdf",
          "timestamp": "2026-08-27T13:11:02+02:00",
          "tree_id": "c3bdb40148223633192c18ee595698a4ba059871",
          "url": "https://github.com/navikt/pdfgenrs/commit/a55ee2e0acfdf5d4d5ec8b49fac0f671fb44b8fd"
        },
        "date": 1787829254220,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 229412,
            "range": "± 15577",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 428749,
            "range": "± 3709",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5622490,
            "range": "± 121751",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91495,
            "range": "± 8803",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7073381,
            "range": "± 246813",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 174031,
            "range": "± 5297",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 221574,
            "range": "± 9507",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 448777,
            "range": "± 3492",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10885,
            "range": "± 178",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18978,
            "range": "± 223",
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
          "id": "457144454812a761792b3d01960de7ab9c937bb0",
          "message": "Merge pull request #439 from tidnav/fix/image-rejection-status-codes\n\nfix(image): return 400/413 instead of 500 for rejected images",
          "timestamp": "2026-08-28T12:36:13+02:00",
          "tree_id": "3c15fb88ea1da7213275331d1b96a5c4cf99a1a8",
          "url": "https://github.com/navikt/pdfgenrs/commit/457144454812a761792b3d01960de7ab9c937bb0"
        },
        "date": 1787913551320,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 212503,
            "range": "± 4845",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 403301,
            "range": "± 12148",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5362234,
            "range": "± 50328",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 86630,
            "range": "± 6934",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6312741,
            "range": "± 68176",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 159665,
            "range": "± 4054",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 215176,
            "range": "± 6129",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 441644,
            "range": "± 2842",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10852,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18228,
            "range": "± 276",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}