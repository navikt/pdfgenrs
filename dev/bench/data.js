window.BENCHMARK_DATA = {
  "lastUpdate": 1788531927943,
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
          "id": "4695adc7d5c870ba6f9512874de26ed9f390314a",
          "message": "Merge pull request #452 from navikt/copilot/speed-up-analyze-task-ci\n\nCI: cache CodeQL dependencies",
          "timestamp": "2026-08-29T06:34:16+02:00",
          "tree_id": "fc068db9243f68c85ff2b38750ae634e55cfac57",
          "url": "https://github.com/navikt/pdfgenrs/commit/4695adc7d5c870ba6f9512874de26ed9f390314a"
        },
        "date": 1787978246401,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 228682,
            "range": "± 15286",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 425532,
            "range": "± 10753",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5218219,
            "range": "± 88957",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 89914,
            "range": "± 8061",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7592077,
            "range": "± 105462",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 167757,
            "range": "± 4314",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 219392,
            "range": "± 9447",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 447596,
            "range": "± 2864",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10593,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18467,
            "range": "± 432",
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
          "id": "2e44099aaa94522599a13f678462d0916d20297c",
          "message": "Merge pull request #457 from navikt/copilot/fix-rfc-9457-error-handling\n\nClarify RFC 9457 error response scope",
          "timestamp": "2026-08-29T21:06:32+02:00",
          "tree_id": "724271606d8c1661bb79c6239500a860e4ca655b",
          "url": "https://github.com/navikt/pdfgenrs/commit/2e44099aaa94522599a13f678462d0916d20297c"
        },
        "date": 1788030577049,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 210459,
            "range": "± 1717",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 399509,
            "range": "± 13252",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5426680,
            "range": "± 67283",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 86274,
            "range": "± 7566",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7800969,
            "range": "± 181796",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 163860,
            "range": "± 4523",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 216698,
            "range": "± 14344",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 439410,
            "range": "± 3638",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10859,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18225,
            "range": "± 548",
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
          "id": "614e1549157a895f20abe76d927b1c54ebcd4cf9",
          "message": "Merge pull request #459 from navikt/copilot/speed-up-analyze-yml\n\nCI: skip CodeQL for documentation changes",
          "timestamp": "2026-08-30T09:08:59+02:00",
          "tree_id": "d0777de41d9c2270b985cd77d62c5ecd30dfa22b",
          "url": "https://github.com/navikt/pdfgenrs/commit/614e1549157a895f20abe76d927b1c54ebcd4cf9"
        },
        "date": 1788073919016,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 225593,
            "range": "± 14185",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 417588,
            "range": "± 4885",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5271121,
            "range": "± 89197",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 90894,
            "range": "± 7271",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7702854,
            "range": "± 111833",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 168052,
            "range": "± 4258",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 219262,
            "range": "± 8313",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 444412,
            "range": "± 4041",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10896,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18746,
            "range": "± 78",
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
          "id": "748f2206463c5f66632726cf4376622f037dbce1",
          "message": "Merge pull request #460 from navikt/copilot/add-dockerignore-file\n\nAdd Docker build context exclusions",
          "timestamp": "2026-08-30T09:26:48+02:00",
          "tree_id": "8c0dfaada641edfc69d94f58e595079e8cc93c8e",
          "url": "https://github.com/navikt/pdfgenrs/commit/748f2206463c5f66632726cf4376622f037dbce1"
        },
        "date": 1788075003368,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 225152,
            "range": "± 9643",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 423117,
            "range": "± 2648",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5295645,
            "range": "± 78613",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91186,
            "range": "± 7470",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 8057470,
            "range": "± 238301",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 173517,
            "range": "± 5759",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 223281,
            "range": "± 10631",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 447626,
            "range": "± 14929",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11313,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 19057,
            "range": "± 459",
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
          "id": "004a41a0756a0308563f65858a915586ed814fd5",
          "message": "Merge pull request #461 from navikt/copilot/validate-dockerfile\n\nAdd Dockerfile validation workflow",
          "timestamp": "2026-08-30T09:42:14+02:00",
          "tree_id": "01c42491adde15a2e93fdb91467e51ab9224a3c6",
          "url": "https://github.com/navikt/pdfgenrs/commit/004a41a0756a0308563f65858a915586ed814fd5"
        },
        "date": 1788075915766,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 212211,
            "range": "± 11889",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 402339,
            "range": "± 3294",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5348170,
            "range": "± 46251",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 86865,
            "range": "± 6125",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7422142,
            "range": "± 33736",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 158705,
            "range": "± 2616",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 224070,
            "range": "± 4716",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 441328,
            "range": "± 3876",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10786,
            "range": "± 466",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18200,
            "range": "± 72",
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
          "id": "668637d6f503ff5ddeb260f5f38159a1e59fecdf",
          "message": "Merge pull request #462 from tidnav/fix/log-level\n\nfix(logging): rename log_level field to level for Elastic",
          "timestamp": "2026-09-02T14:18:38+02:00",
          "tree_id": "750222e54b0f2542d74878bf4eab22cdcee6c0a5",
          "url": "https://github.com/navikt/pdfgenrs/commit/668637d6f503ff5ddeb260f5f38159a1e59fecdf"
        },
        "date": 1788351722644,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 152493,
            "range": "± 5044",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 338792,
            "range": "± 8006",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4446013,
            "range": "± 69944",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 81011,
            "range": "± 7069",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6869840,
            "range": "± 79618",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 122331,
            "range": "± 7374",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 177134,
            "range": "± 7589",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 371210,
            "range": "± 5268",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 12264,
            "range": "± 106",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18445,
            "range": "± 648",
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
          "id": "9550e89dc83a23acfbb587128c7472f8be83a87f",
          "message": "Merge pull request #466 from navikt/dependabot/github_actions/github/codeql-action-4.37.9\n\nchore(deps): bump github/codeql-action from 4.37.8 to 4.37.9",
          "timestamp": "2026-09-04T15:44:39+02:00",
          "tree_id": "3f5d3842613a4e22be9911cd9734c49c18934477",
          "url": "https://github.com/navikt/pdfgenrs/commit/9550e89dc83a23acfbb587128c7472f8be83a87f"
        },
        "date": 1788530121918,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 211109,
            "range": "± 9220",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 398807,
            "range": "± 3003",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5294499,
            "range": "± 48275",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 86314,
            "range": "± 7482",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7441602,
            "range": "± 50598",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 161820,
            "range": "± 2347",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 238431,
            "range": "± 4466",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 440289,
            "range": "± 1867",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10855,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18395,
            "range": "± 84",
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
          "id": "80f0da4796beaff9350c3a1daa0a1f623eef0e6f",
          "message": "Merge pull request #465 from navikt/dependabot/cargo/ironpress-1.6.0\n\nchore(deps): bump ironpress from 1.5.3 to 1.6.0",
          "timestamp": "2026-09-04T15:44:20+02:00",
          "tree_id": "6b2b73432893c30156289110536cd0b168940df2",
          "url": "https://github.com/navikt/pdfgenrs/commit/80f0da4796beaff9350c3a1daa0a1f623eef0e6f"
        },
        "date": 1788530139722,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 230508,
            "range": "± 13728",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 427965,
            "range": "± 2426",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5354959,
            "range": "± 136174",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91835,
            "range": "± 6548",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7876360,
            "range": "± 213383",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 170104,
            "range": "± 5745",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 237160,
            "range": "± 6041",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 453614,
            "range": "± 6443",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10909,
            "range": "± 114",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 19020,
            "range": "± 88",
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
          "id": "9cbe622c4f5e69be31ce56986bbf5b2599df638c",
          "message": "Merge pull request #464 from navikt/dependabot/cargo/uuid-1.26.0\n\nchore(deps): bump uuid from 1.24.1 to 1.26.0",
          "timestamp": "2026-09-04T15:44:55+02:00",
          "tree_id": "16ce83aff6583ab203e29af4972166a510f32516",
          "url": "https://github.com/navikt/pdfgenrs/commit/9cbe622c4f5e69be31ce56986bbf5b2599df638c"
        },
        "date": 1788530181636,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 230036,
            "range": "± 3777",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 430310,
            "range": "± 17591",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5454769,
            "range": "± 73499",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91539,
            "range": "± 6604",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7814380,
            "range": "± 91984",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 173596,
            "range": "± 6464",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 246348,
            "range": "± 8038",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 458669,
            "range": "± 6560",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10978,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18784,
            "range": "± 96",
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
          "id": "091ee9bf37ed63f8d331ebd2c62a84f26d0e3fd4",
          "message": "Merge pull request #463 from navikt/dependabot/docker/clux/muslrust-01ce665\n\nchore(deps): bump clux/muslrust from `4edc98b` to `01ce665`",
          "timestamp": "2026-09-04T16:22:00+02:00",
          "tree_id": "4808c61f8a0af0ef865f6931fde18b4761013a49",
          "url": "https://github.com/navikt/pdfgenrs/commit/091ee9bf37ed63f8d331ebd2c62a84f26d0e3fd4"
        },
        "date": 1788531919186,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 234120,
            "range": "± 18265",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 433984,
            "range": "± 3491",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5293098,
            "range": "± 47388",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92242,
            "range": "± 7181",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7770014,
            "range": "± 194119",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 183000,
            "range": "± 5661",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 258766,
            "range": "± 9413",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 458277,
            "range": "± 15329",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11084,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 19161,
            "range": "± 97",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}