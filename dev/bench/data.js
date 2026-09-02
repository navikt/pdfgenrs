window.BENCHMARK_DATA = {
  "lastUpdate": 1788351732569,
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
          "id": "d13452270d957f0f6921241eb591bc7919e9419f",
          "message": "Merge pull request #447 from navikt/copilot/enforce-rust-version-ci\n\nEnforce declared Rust version in CI",
          "timestamp": "2026-08-28T19:47:34+02:00",
          "tree_id": "4b05c2df2b3801a5bc71a1d094411b630aecb378",
          "url": "https://github.com/navikt/pdfgenrs/commit/d13452270d957f0f6921241eb591bc7919e9419f"
        },
        "date": 1787939456247,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 225675,
            "range": "± 13254",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 425963,
            "range": "± 2899",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5323198,
            "range": "± 88994",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 90836,
            "range": "± 7988",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 8527952,
            "range": "± 269487",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 165333,
            "range": "± 5426",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 218666,
            "range": "± 9790",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 447413,
            "range": "± 5818",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10911,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18736,
            "range": "± 114",
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
          "id": "2b0e6752a5722e3e2e2a925e182321c90efe7e70",
          "message": "Merge pull request #448 from navikt/copilot/fix-github-actions-job-formatting\n\nCI: install rustfmt for Rust 1.96.0 formatting check",
          "timestamp": "2026-08-28T20:20:55+02:00",
          "tree_id": "4dc47ef2cf136fb81a7042018ddac137857099fc",
          "url": "https://github.com/navikt/pdfgenrs/commit/2b0e6752a5722e3e2e2a925e182321c90efe7e70"
        },
        "date": 1787941444140,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 212162,
            "range": "± 16233",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 397884,
            "range": "± 2567",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5419184,
            "range": "± 82526",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 86948,
            "range": "± 7444",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 8235726,
            "range": "± 186875",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 160119,
            "range": "± 4337",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 215504,
            "range": "± 8343",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 438859,
            "range": "± 8533",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10928,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18328,
            "range": "± 299",
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
          "id": "e3c732aee638147af2de13647b901a392aec0b86",
          "message": "Merge pull request #449 from navikt/copilot/speed-up-cargo-test-ci\n\nCI: scope Rust cache to toolchain",
          "timestamp": "2026-08-28T21:17:33+02:00",
          "tree_id": "2e9a874ed5d11b0f02d5fca31e8e54a2ca8d9e34",
          "url": "https://github.com/navikt/pdfgenrs/commit/e3c732aee638147af2de13647b901a392aec0b86"
        },
        "date": 1787944847376,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 225033,
            "range": "± 1882",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 422749,
            "range": "± 4364",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5207325,
            "range": "± 41813",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91077,
            "range": "± 6257",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7610232,
            "range": "± 68578",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 163889,
            "range": "± 5387",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 218034,
            "range": "± 6301",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 445706,
            "range": "± 2819",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10785,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18513,
            "range": "± 135",
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
          "id": "afd5041f742796144c7bc13141a1390255741cad",
          "message": "Merge pull request #451 from navikt/copilot/speed-up-ci-task9\n\nCI: speed up Cargo test task",
          "timestamp": "2026-08-29T06:28:44+02:00",
          "tree_id": "e0eb26d460b46db5b09eaff96b28355f7383a2f2",
          "url": "https://github.com/navikt/pdfgenrs/commit/afd5041f742796144c7bc13141a1390255741cad"
        },
        "date": 1787977912479,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 89719,
            "range": "± 3633",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 191566,
            "range": "± 6971",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 3096988,
            "range": "± 40051",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 49148,
            "range": "± 3556",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 4725090,
            "range": "± 71919",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 72864,
            "range": "± 2104",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 109616,
            "range": "± 2076",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 220094,
            "range": "± 3599",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 9225,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 13490,
            "range": "± 265",
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
      }
    ]
  }
}