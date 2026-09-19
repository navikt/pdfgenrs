window.BENCHMARK_DATA = {
  "lastUpdate": 1789817497805,
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
          "id": "8133dee2f7b9b1fe9697a7e0da023a7778e810ec",
          "message": "Merge pull request #476 from navikt/MikAoJk-patch-2\n\nchore: Specify version for cargo-auditable installation",
          "timestamp": "2026-09-15T18:01:03+02:00",
          "tree_id": "2a32b074417e207f1a7a68fff6bc6d4e13248b64",
          "url": "https://github.com/navikt/pdfgenrs/commit/8133dee2f7b9b1fe9697a7e0da023a7778e810ec"
        },
        "date": 1789488257322,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 227969,
            "range": "± 13421",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 425308,
            "range": "± 16158",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5461506,
            "range": "± 58204",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91693,
            "range": "± 6918",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 8280407,
            "range": "± 118275",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 161518,
            "range": "± 4798",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 238215,
            "range": "± 3950",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 449305,
            "range": "± 10124",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10924,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18907,
            "range": "± 102",
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
          "id": "a879ae7a28110ff6de81a8646a45248fa7ebbf12",
          "message": "Merge pull request #479 from navikt/MikAoJk-patch-2\n\nRestore rustls dependency in Cargo.toml",
          "timestamp": "2026-09-15T19:00:40+02:00",
          "tree_id": "e7c36c779827640ec401c2d5c93f6cb2319ba8df",
          "url": "https://github.com/navikt/pdfgenrs/commit/a879ae7a28110ff6de81a8646a45248fa7ebbf12"
        },
        "date": 1789491840794,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 150338,
            "range": "± 10237",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 333806,
            "range": "± 4236",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4441224,
            "range": "± 193733",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 81629,
            "range": "± 5960",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6964652,
            "range": "± 411108",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 116740,
            "range": "± 6571",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 180913,
            "range": "± 7471",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 370710,
            "range": "± 17564",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 12266,
            "range": "± 446",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18258,
            "range": "± 1021",
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
          "id": "0ccdb78f3da1d917be61743f01b99776ca918368",
          "message": "Merge pull request #480 from navikt/copilot/expand-docker-ci-html-generation\n\nAdd Docker HTML API validation",
          "timestamp": "2026-09-15T20:04:29+02:00",
          "tree_id": "afb674ab0cd9f27eb27e7b9a448d4142f3c9ae11",
          "url": "https://github.com/navikt/pdfgenrs/commit/0ccdb78f3da1d917be61743f01b99776ca918368"
        },
        "date": 1789495655540,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 228577,
            "range": "± 13600",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 423219,
            "range": "± 2793",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5291012,
            "range": "± 45606",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91697,
            "range": "± 7113",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7973553,
            "range": "± 156220",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 169705,
            "range": "± 6400",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 245906,
            "range": "± 7541",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 455395,
            "range": "± 9916",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11128,
            "range": "± 273",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 19162,
            "range": "± 98",
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
          "id": "fff16df5d8ce6353b90eefe33935677935359ee2",
          "message": "Merge pull request #481 from navikt/copilot/add-api-error-boundary\n\nUse Problem Details for framework errors",
          "timestamp": "2026-09-16T06:42:32+02:00",
          "tree_id": "a4bfc9e782c790e436d27471b7f04545c6e9225a",
          "url": "https://github.com/navikt/pdfgenrs/commit/fff16df5d8ce6353b90eefe33935677935359ee2"
        },
        "date": 1789533969120,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 125959,
            "range": "± 7917",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 278337,
            "range": "± 18432",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 3694691,
            "range": "± 181775",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 63713,
            "range": "± 3788",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6563159,
            "range": "± 112076",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 99893,
            "range": "± 4168",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 153143,
            "range": "± 4760",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 309297,
            "range": "± 15956",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 9990,
            "range": "± 865",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 14955,
            "range": "± 987",
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
          "id": "bd60e4b9bc7efa81e3c3680f41a8994addac3482",
          "message": "Merge pull request #482 from navikt/bumpdeps\n\nchore: bump the deps",
          "timestamp": "2026-09-16T09:16:12+02:00",
          "tree_id": "410022c67c365e0d0fb916d85b309f89f06f414f",
          "url": "https://github.com/navikt/pdfgenrs/commit/bd60e4b9bc7efa81e3c3680f41a8994addac3482"
        },
        "date": 1789543190299,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 229356,
            "range": "± 9350",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 427688,
            "range": "± 3270",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5381105,
            "range": "± 31345",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91662,
            "range": "± 6231",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7537852,
            "range": "± 67329",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 170122,
            "range": "± 4304",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 243910,
            "range": "± 6635",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 452778,
            "range": "± 4809",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11169,
            "range": "± 354",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18954,
            "range": "± 140",
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
      }
    ]
  }
}