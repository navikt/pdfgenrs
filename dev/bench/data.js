window.BENCHMARK_DATA = {
  "lastUpdate": 1785506376323,
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
          "id": "0f9bc02a0961c32e253998bb3f5b3aff9a763379",
          "message": "Merge pull request #374 from navikt/copilot/fix-svg-attribute-matching\n\nfix: word boundary check in extract_svg_attr to prevent partial attribute name matches",
          "timestamp": "2026-07-31T09:37:45+02:00",
          "tree_id": "78d4f1067790c6d39acf4c10365edd5c4359daa8",
          "url": "https://github.com/navikt/pdfgenrs/commit/0f9bc02a0961c32e253998bb3f5b3aff9a763379"
        },
        "date": 1785483548199,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 165789,
            "range": "± 6611",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 362570,
            "range": "± 16857",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 110305,
            "range": "± 7319",
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
          "id": "067bdb073779a84040b5d80fdd92cebc2e19af70",
          "message": "Merge pull request #376 from navikt/copilot/allow-clippy-too-many-arguments\n\nrefactor: introduce CompileRequest struct to eliminate too_many_arguments suppression",
          "timestamp": "2026-07-31T10:12:02+02:00",
          "tree_id": "749e811973f426c583ae1422558c59d9ac85d3a7",
          "url": "https://github.com/navikt/pdfgenrs/commit/067bdb073779a84040b5d80fdd92cebc2e19af70"
        },
        "date": 1785485611353,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 230599,
            "range": "± 15131",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 427437,
            "range": "± 4336",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 154325,
            "range": "± 4207",
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
          "id": "6ccb6ada29f648358c61fbd79db01a1efcd311b0",
          "message": "Merge pull request #378 from navikt/copilot/expand-benchmarks-for-typst\n\nExpand criterion benchmarks: concurrent compilation, large JSON payloads, image format variants",
          "timestamp": "2026-07-31T10:56:27+02:00",
          "tree_id": "a50a87484f3e30816151f89f7ca41a1d02d31dfc",
          "url": "https://github.com/navikt/pdfgenrs/commit/6ccb6ada29f648358c61fbd79db01a1efcd311b0"
        },
        "date": 1785488343317,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 233138,
            "range": "± 20840",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 425106,
            "range": "± 1661",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5179836,
            "range": "± 48975",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent/2",
            "value": 146173,
            "range": "± 3995",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent/4",
            "value": 82882,
            "range": "± 8425",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent/8",
            "value": 90810,
            "range": "± 6910",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 156848,
            "range": "± 4060",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 216745,
            "range": "± 8663",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 363536,
            "range": "± 5735",
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
          "id": "fbc11ed57c4c35ca7c2e7535b3181e2e5afbddc9",
          "message": "Merge pull request #377 from navikt/copilot/add-request-id-to-error-responses\n\nfeat: include request_id in RFC 9457 error responses",
          "timestamp": "2026-07-31T11:01:19+02:00",
          "tree_id": "5a4b55f23134b5d35c3d5a9d0ff24b5b0cb001ba",
          "url": "https://github.com/navikt/pdfgenrs/commit/fbc11ed57c4c35ca7c2e7535b3181e2e5afbddc9"
        },
        "date": 1785488649431,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 232035,
            "range": "± 11059",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 424233,
            "range": "± 9570",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5130369,
            "range": "± 61766",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent/2",
            "value": 145353,
            "range": "± 3651",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent/4",
            "value": 85015,
            "range": "± 11284",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent/8",
            "value": 91804,
            "range": "± 6982",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 159072,
            "range": "± 5456",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 211022,
            "range": "± 7341",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 369389,
            "range": "± 2216",
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
          "id": "e0ccc0137f2dd6ff3160453b0983dee8f058ed9d",
          "message": "Merge pull request #379 from navikt/copilot/typst-to-pdf-concurrent-8\n\nbench: only run typst_to_pdf_concurrent at concurrency 8",
          "timestamp": "2026-07-31T11:52:18+02:00",
          "tree_id": "60ed188b1e0b6269a27088efbe461d8c95cdf424",
          "url": "https://github.com/navikt/pdfgenrs/commit/e0ccc0137f2dd6ff3160453b0983dee8f058ed9d"
        },
        "date": 1785491679794,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 233239,
            "range": "± 16791",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 426448,
            "range": "± 3151",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5229899,
            "range": "± 32153",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent/8",
            "value": 92792,
            "range": "± 7965",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 163179,
            "range": "± 2993",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 211253,
            "range": "± 7098",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 367434,
            "range": "± 7511",
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
          "id": "ac472cb10ed7621ac04c5a21c72723910a728814",
          "message": "Merge pull request #380 from navikt/copilot/bench-typst-to-pdf-concurrent\n\nDrop /8 suffix from bench_typst_to_pdf_concurrent benchmark name",
          "timestamp": "2026-07-31T12:11:32+02:00",
          "tree_id": "848dff4742f8251529c528bf987e9902d1fabd29",
          "url": "https://github.com/navikt/pdfgenrs/commit/ac472cb10ed7621ac04c5a21c72723910a728814"
        },
        "date": 1785492820219,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 213426,
            "range": "± 10136",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 396095,
            "range": "± 9103",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5158289,
            "range": "± 242089",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent/typst_to_pdf_concurrent",
            "value": 85367,
            "range": "± 7734",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 150180,
            "range": "± 2006",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 210992,
            "range": "± 5273",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 351737,
            "range": "± 5235",
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
          "id": "79245ed648c647e304082def0d9d52fef465639a",
          "message": "Merge pull request #382 from navikt/copilot/no-sub-list-typst-to-pdf-concurrent\n\nbench: flatten typst_to_pdf_concurrent into a top-level benchmark",
          "timestamp": "2026-07-31T12:29:04+02:00",
          "tree_id": "c61df4786ea292266a387985342fe80d10f01a71",
          "url": "https://github.com/navikt/pdfgenrs/commit/79245ed648c647e304082def0d9d52fef465639a"
        },
        "date": 1785493890222,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 229939,
            "range": "± 12236",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 422116,
            "range": "± 4949",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5323455,
            "range": "± 62271",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91600,
            "range": "± 8482",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 175242,
            "range": "± 4963",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 213146,
            "range": "± 10828",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 363108,
            "range": "± 2341",
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
          "id": "d04ce8af338ab4f5438f697719a2ffe5c1baa7b1",
          "message": "Merge pull request #381 from navikt/copilot/expose-comemo-eviction-threshold\n\nExpose COMEMO_EVICTION_THRESHOLD as env-var config option",
          "timestamp": "2026-07-31T12:54:18+02:00",
          "tree_id": "d1ec5ab078cf689e891be57518d712a4b9e8beb2",
          "url": "https://github.com/navikt/pdfgenrs/commit/d04ce8af338ab4f5438f697719a2ffe5c1baa7b1"
        },
        "date": 1785495391979,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 231351,
            "range": "± 13155",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 427549,
            "range": "± 2798",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5254082,
            "range": "± 180324",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92372,
            "range": "± 7842",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 163308,
            "range": "± 4054",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 215531,
            "range": "± 7710",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 364819,
            "range": "± 2603",
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
          "id": "20a80e25ae5df0f42a11cd7a47c99a7e9db7ce12",
          "message": "Merge pull request #384 from navikt/copilot/structured-logging-improvement\n\nfeat: single structured startup summary log",
          "timestamp": "2026-07-31T13:43:19+02:00",
          "tree_id": "908fae0c9c50482812b7fc92481a45b4b47bdcfb",
          "url": "https://github.com/navikt/pdfgenrs/commit/20a80e25ae5df0f42a11cd7a47c99a7e9db7ce12"
        },
        "date": 1785498331424,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 230714,
            "range": "± 10384",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 422350,
            "range": "± 15223",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5164647,
            "range": "± 105125",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91845,
            "range": "± 6518",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 161810,
            "range": "± 4406",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 215728,
            "range": "± 11139",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 366018,
            "range": "± 5385",
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
          "id": "9b439328853ce45348a8065de804229a1c6ddb34",
          "message": "Merge pull request #390 from navikt/dependabot/github_actions/github/codeql-action-4.37.3\n\nchore(deps): bump github/codeql-action from 4.37.1 to 4.37.3",
          "timestamp": "2026-07-31T15:57:19+02:00",
          "tree_id": "73f1ce20f2698d956a4a4c61eddb1ddd815776c8",
          "url": "https://github.com/navikt/pdfgenrs/commit/9b439328853ce45348a8065de804229a1c6ddb34"
        },
        "date": 1785506371463,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 211016,
            "range": "± 2520",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 396320,
            "range": "± 7126",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5189563,
            "range": "± 63088",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 86752,
            "range": "± 10696",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 149924,
            "range": "± 2221",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 210970,
            "range": "± 8806",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 351534,
            "range": "± 5339",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}