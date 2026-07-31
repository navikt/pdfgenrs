window.BENCHMARK_DATA = {
  "lastUpdate": 1785506582229,
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
          "id": "3048309e80ddd553fe2c6a1d733b185d98980721",
          "message": "Merge pull request #391 from navikt/dependabot/cargo/serde_json-1.0.151\n\nchore(deps): bump serde_json from 1.0.150 to 1.0.151",
          "timestamp": "2026-07-31T15:57:33+02:00",
          "tree_id": "ef5689c31a355a2658241f2f4055ae33b7cfb711",
          "url": "https://github.com/navikt/pdfgenrs/commit/3048309e80ddd553fe2c6a1d733b185d98980721"
        },
        "date": 1785506489714,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 184934,
            "range": "± 7235",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 405609,
            "range": "± 6130",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5125183,
            "range": "± 27201",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 96325,
            "range": "± 6068",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 132303,
            "range": "± 6424",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 199912,
            "range": "± 8566",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 329635,
            "range": "± 4472",
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
          "id": "e99f3b768a8306ad4d94e54727e40e7f1f00147c",
          "message": "Merge pull request #389 from navikt/dependabot/cargo/tokio-1.53.1\n\nchore(deps): bump tokio from 1.53.0 to 1.53.1",
          "timestamp": "2026-07-31T15:57:53+02:00",
          "tree_id": "143d1e854f2a201e4d94b3b5b6bfaf02485540da",
          "url": "https://github.com/navikt/pdfgenrs/commit/e99f3b768a8306ad4d94e54727e40e7f1f00147c"
        },
        "date": 1785506491833,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 126819,
            "range": "± 4532",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 282241,
            "range": "± 17153",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 3581955,
            "range": "± 173186",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 66551,
            "range": "± 7110",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 92972,
            "range": "± 3914",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 144706,
            "range": "± 10517",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 235428,
            "range": "± 14538",
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
          "id": "800022c6e104d32010d21d167411d3138ff271ea",
          "message": "Merge pull request #388 from navikt/dependabot/cargo/typst-pdf-0.15.1\n\nchore(deps): bump typst-pdf from 0.15.0 to 0.15.1",
          "timestamp": "2026-07-31T15:58:05+02:00",
          "tree_id": "9851f02967b6537f6baef2d40ce9c8b2517be30c",
          "url": "https://github.com/navikt/pdfgenrs/commit/800022c6e104d32010d21d167411d3138ff271ea"
        },
        "date": 1785506541118,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 226458,
            "range": "± 2265",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 423505,
            "range": "± 2585",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5133257,
            "range": "± 101649",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92458,
            "range": "± 5253",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 155263,
            "range": "± 4686",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 207700,
            "range": "± 7418",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 366597,
            "range": "± 1629",
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
          "id": "ee761867d476d5eafbefb95f331ce9093fffb058",
          "message": "Merge pull request #385 from navikt/dependabot/cargo/anyhow-1.0.104\n\nchore(deps): bump anyhow from 1.0.103 to 1.0.104",
          "timestamp": "2026-07-31T15:58:21+02:00",
          "tree_id": "0307594a9591acd0019fd0c1ba746478a53257ee",
          "url": "https://github.com/navikt/pdfgenrs/commit/ee761867d476d5eafbefb95f331ce9093fffb058"
        },
        "date": 1785506557132,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 215033,
            "range": "± 7809",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 406290,
            "range": "± 1737",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5393606,
            "range": "± 37582",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 86910,
            "range": "± 6096",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 150646,
            "range": "± 2655",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 213621,
            "range": "± 5195",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 362160,
            "range": "± 2019",
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
          "id": "f1e8174500759ad343871523afc3edd93ce46e35",
          "message": "Merge pull request #387 from navikt/dependabot/cargo/time-0.3.54\n\nchore(deps): bump time from 0.3.53 to 0.3.54",
          "timestamp": "2026-07-31T15:58:35+02:00",
          "tree_id": "0d21aac80fd370b18f6e5125945a35f7c0f51306",
          "url": "https://github.com/navikt/pdfgenrs/commit/f1e8174500759ad343871523afc3edd93ce46e35"
        },
        "date": 1785506576284,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 185303,
            "range": "± 6627",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 398606,
            "range": "± 3841",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5123003,
            "range": "± 44801",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 96924,
            "range": "± 5525",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 134044,
            "range": "± 6516",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 202289,
            "range": "± 9615",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 329452,
            "range": "± 63295",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}