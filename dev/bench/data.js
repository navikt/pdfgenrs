window.BENCHMARK_DATA = {
  "lastUpdate": 1787941451962,
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
          "id": "3d78b51b4b2e83db1068c694838ea83f770b45ea",
          "message": "Merge pull request #444 from navikt/dependabot/github_actions/github/codeql-action-4.37.8\n\nchore(deps): bump github/codeql-action from 4.37.7 to 4.37.8",
          "timestamp": "2026-08-28T15:40:21+02:00",
          "tree_id": "05c58702d03a243bef5a20eb88484be090225a74",
          "url": "https://github.com/navikt/pdfgenrs/commit/3d78b51b4b2e83db1068c694838ea83f770b45ea"
        },
        "date": 1787924605907,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 241987,
            "range": "± 6861",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 456530,
            "range": "± 8227",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 6208178,
            "range": "± 170969",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 100875,
            "range": "± 7334",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7014869,
            "range": "± 54355",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 178007,
            "range": "± 10101",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 247837,
            "range": "± 4826",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 502955,
            "range": "± 2599",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 12551,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 20944,
            "range": "± 137",
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
          "id": "3e7e887d11f59e6580ea18f72d6f17fefc82f3da",
          "message": "Merge pull request #441 from navikt/dependabot/cargo/uuid-1.24.1\n\nchore(deps): bump uuid from 1.24.0 to 1.24.1",
          "timestamp": "2026-08-28T15:40:44+02:00",
          "tree_id": "cbc0b6262ca440021ce16a05de490c0ee1c0d221",
          "url": "https://github.com/navikt/pdfgenrs/commit/3e7e887d11f59e6580ea18f72d6f17fefc82f3da"
        },
        "date": 1787924619715,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 90991,
            "range": "± 4029",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 195896,
            "range": "± 5372",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 3106568,
            "range": "± 11723",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 46327,
            "range": "± 3616",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 4015126,
            "range": "± 98419",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 76270,
            "range": "± 2445",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 110830,
            "range": "± 2572",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 221005,
            "range": "± 1424",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 9379,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 13534,
            "range": "± 30",
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
          "id": "8ee2050c6a1e40a69c680da2d2ff3bd41e072fdb",
          "message": "Merge pull request #440 from navikt/dependabot/docker/distroless/static-debian13-1c2c046\n\nchore(deps): bump distroless/static-debian13 from `f7f8f72` to `1c2c046`",
          "timestamp": "2026-08-28T15:40:33+02:00",
          "tree_id": "0a7f10dc0de1c8f0dcfa99707236bff918c560aa",
          "url": "https://github.com/navikt/pdfgenrs/commit/8ee2050c6a1e40a69c680da2d2ff3bd41e072fdb"
        },
        "date": 1787924628818,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 226974,
            "range": "± 11125",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 421718,
            "range": "± 2513",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5345848,
            "range": "± 31232",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91862,
            "range": "± 7697",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6516796,
            "range": "± 236537",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 171066,
            "range": "± 7157",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 223849,
            "range": "± 8710",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 447655,
            "range": "± 3138",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10909,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18737,
            "range": "± 122",
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
          "id": "94480b169bc1953fc8e5bd5e7cbb690c18706aaa",
          "message": "Merge pull request #442 from navikt/dependabot/cargo/axum-test-21.1.0\n\nchore(deps): bump axum-test from 21.0.0 to 21.1.0",
          "timestamp": "2026-08-28T15:40:59+02:00",
          "tree_id": "2722a25ecdeba8a25b1179b0fd65d114efbc74af",
          "url": "https://github.com/navikt/pdfgenrs/commit/94480b169bc1953fc8e5bd5e7cbb690c18706aaa"
        },
        "date": 1787924970599,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 232117,
            "range": "± 7626",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 433431,
            "range": "± 7089",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5500298,
            "range": "± 71796",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 93126,
            "range": "± 8619",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 6564658,
            "range": "± 101898",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 176839,
            "range": "± 5882",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 230000,
            "range": "± 10621",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 452161,
            "range": "± 2387",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10887,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18717,
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
          "id": "0cbb9991810878dbc6c120ab66017a7c18ad2691",
          "message": "Merge pull request #443 from navikt/dependabot/cargo/ironpress-1.5.3\n\nchore(deps): bump ironpress from 1.5.2 to 1.5.3",
          "timestamp": "2026-08-28T15:41:34+02:00",
          "tree_id": "ead13c60b68967bba74cea7ad72e9771935a7939",
          "url": "https://github.com/navikt/pdfgenrs/commit/0cbb9991810878dbc6c120ab66017a7c18ad2691"
        },
        "date": 1787925045394,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 234591,
            "range": "± 16487",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 433148,
            "range": "± 21188",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5780214,
            "range": "± 243821",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92770,
            "range": "± 10744",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 9472790,
            "range": "± 1362201",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 177529,
            "range": "± 8184",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 240640,
            "range": "± 10661",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 455522,
            "range": "± 13083",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 11133,
            "range": "± 173",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18981,
            "range": "± 133",
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
          "id": "8ffe2d168c6f5744d6c2a26efd00ea5bc26e41fd",
          "message": "Merge pull request #445 from navikt/copilot/apply-cargo-locked-to-clippy-tests\n\nEnforce Cargo.lock in CI",
          "timestamp": "2026-08-28T19:30:12+02:00",
          "tree_id": "f0215c1bbb5c0d9734de196889be59b21a8c1488",
          "url": "https://github.com/navikt/pdfgenrs/commit/8ffe2d168c6f5744d6c2a26efd00ea5bc26e41fd"
        },
        "date": 1787938402565,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 215106,
            "range": "± 7984",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 407141,
            "range": "± 2957",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5352004,
            "range": "± 36137",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 87031,
            "range": "± 7070",
            "unit": "ns/iter"
          },
          {
            "name": "html_to_pdf",
            "value": 7527334,
            "range": "± 52777",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 162576,
            "range": "± 2971",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 215107,
            "range": "± 4662",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 445508,
            "range": "± 2946",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_simple",
            "value": 10941,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_html_with_data",
            "value": 18401,
            "range": "± 64",
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
      }
    ]
  }
}