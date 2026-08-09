window.BENCHMARK_DATA = {
  "lastUpdate": 1786256901685,
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
          "id": "e3a174ada131b7224e32e5a89c56fa5456b222e3",
          "message": "Merge pull request #392 from navikt/copilot/add-webp-svg-image-to-pdf-route-test\n\nAdd WebP and SVG success tests for image-to-PDF route",
          "timestamp": "2026-08-01T09:04:57+02:00",
          "tree_id": "a383c267878cf120e82540842dfa0d609b90533f",
          "url": "https://github.com/navikt/pdfgenrs/commit/e3a174ada131b7224e32e5a89c56fa5456b222e3"
        },
        "date": 1785568027479,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 226422,
            "range": "± 18107",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 413873,
            "range": "± 8539",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5033394,
            "range": "± 46737",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91797,
            "range": "± 6802",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 156347,
            "range": "± 4216",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 207400,
            "range": "± 5871",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 358904,
            "range": "± 3148",
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
          "id": "e6e2526dac72389916a7b60d466832230221524b",
          "message": "Merge pull request #393 from navikt/copilot/review-repository-enhancements-bbd8fdd9-70d4-4b78-b4fa-aedaaeb918fb\n\nrefactor: simplify svg viewBox parsing and remove redundant to_str() call",
          "timestamp": "2026-08-03T19:08:03+02:00",
          "tree_id": "f7bc9d26dbecd9a26b1d7a9bbccc028e599af40a",
          "url": "https://github.com/navikt/pdfgenrs/commit/e6e2526dac72389916a7b60d466832230221524b"
        },
        "date": 1785777017551,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 213469,
            "range": "± 8367",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 397476,
            "range": "± 39785",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5203561,
            "range": "± 75303",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 87169,
            "range": "± 7440",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 148584,
            "range": "± 3431",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 209013,
            "range": "± 7223",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 353875,
            "range": "± 4475",
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
          "id": "c4ad9f9ac21e6ee5d8420310c268861bbbd5ceef",
          "message": "Merge pull request #394 from navikt/copilot/review-repository-enhancements-a8176ca8-9d60-485c-bcde-9592053e661f\n\nrefactor: avoid unnecessary allocations in error formatting and middleware",
          "timestamp": "2026-08-03T19:31:36+02:00",
          "tree_id": "b2ec0a3b4febda7203bc49f32e9fa3a8b3f21c52",
          "url": "https://github.com/navikt/pdfgenrs/commit/c4ad9f9ac21e6ee5d8420310c268861bbbd5ceef"
        },
        "date": 1785778440044,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 192477,
            "range": "± 1886",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 400567,
            "range": "± 5948",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5292520,
            "range": "± 71461",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 93088,
            "range": "± 7247",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 138686,
            "range": "± 3307",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 204595,
            "range": "± 5323",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 334622,
            "range": "± 6142",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "test@example.com",
            "name": "Test User"
          },
          "committer": {
            "email": "test@example.com",
            "name": "Test User"
          },
          "distinct": true,
          "id": "6e01c0b898b123f8f6779b0e5482596a09451a2f",
          "message": "small fixes",
          "timestamp": "2026-08-04T20:24:54+02:00",
          "tree_id": "211a9844ec0cb5e743933794ac9c2992218e6829",
          "url": "https://github.com/navikt/pdfgenrs/commit/6e01c0b898b123f8f6779b0e5482596a09451a2f"
        },
        "date": 1785868041389,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 226114,
            "range": "± 9565",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 417447,
            "range": "± 5971",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5189472,
            "range": "± 71958",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 90942,
            "range": "± 5753",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 154336,
            "range": "± 4536",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 210410,
            "range": "± 10899",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 359505,
            "range": "± 3413",
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
          "id": "76a7883dd681fb9288d27d72a6809f8af7f8ba52",
          "message": "Merge pull request #395 from navikt/dependabot/github_actions/release-drafter/release-drafter-7.7.0\n\nchore(deps): bump release-drafter/release-drafter from 7.6.0 to 7.7.0",
          "timestamp": "2026-08-07T15:17:15+02:00",
          "tree_id": "aa62e7c78e8aa6433350dcc97ca6bd139a3b14c1",
          "url": "https://github.com/navikt/pdfgenrs/commit/76a7883dd681fb9288d27d72a6809f8af7f8ba52"
        },
        "date": 1786108788686,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 228258,
            "range": "± 1786",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 420665,
            "range": "± 1870",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5183609,
            "range": "± 76768",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 91755,
            "range": "± 6562",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 163358,
            "range": "± 4735",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 213392,
            "range": "± 7895",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 369128,
            "range": "± 2694",
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
          "id": "287cb1343ad25d8783f95bf318e7a1c0bf9ac863",
          "message": "Merge pull request #396 from navikt/dependabot/github_actions/github/codeql-action-4.37.4\n\nchore(deps): bump github/codeql-action from 4.37.3 to 4.37.4",
          "timestamp": "2026-08-07T15:17:29+02:00",
          "tree_id": "02c7a0322a68fdf3cc3f31615ccacef3d06b7104",
          "url": "https://github.com/navikt/pdfgenrs/commit/287cb1343ad25d8783f95bf318e7a1c0bf9ac863"
        },
        "date": 1786108791898,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 232268,
            "range": "± 4107",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 423576,
            "range": "± 2474",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5555400,
            "range": "± 66029",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92345,
            "range": "± 7592",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 160880,
            "range": "± 4723",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 214402,
            "range": "± 7270",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 368567,
            "range": "± 1562",
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
          "id": "ef2bc274250fd708a3e97a9dec139d9cef17f5a7",
          "message": "Merge pull request #399 from navikt/copilot/fix-vp8l-signature-verification\n\nfix: verify VP8L signature byte before parsing dimensions",
          "timestamp": "2026-08-09T07:54:25+02:00",
          "tree_id": "4b9b98c7545c358156057f1ee8fa382995303a0a",
          "url": "https://github.com/navikt/pdfgenrs/commit/ef2bc274250fd708a3e97a9dec139d9cef17f5a7"
        },
        "date": 1786255012689,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 217262,
            "range": "± 7211",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 399998,
            "range": "± 2973",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5185188,
            "range": "± 111295",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 87820,
            "range": "± 9000",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 152011,
            "range": "± 3347",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 211743,
            "range": "± 11733",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 362762,
            "range": "± 2183",
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
          "id": "483c55dd5c04621c0628081b0d62ec98dcc2e903",
          "message": "Merge pull request #398 from navikt/copilot/propagate-opentelemetry-span-context\n\nPropagate OTel span context into spawn_blocking in compile_blocking",
          "timestamp": "2026-08-09T07:55:10+02:00",
          "tree_id": "e9d65de6b5dbe72ec8f151acb58764a5cdcc19ea",
          "url": "https://github.com/navikt/pdfgenrs/commit/483c55dd5c04621c0628081b0d62ec98dcc2e903"
        },
        "date": 1786255047905,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 232372,
            "range": "± 15769",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 418005,
            "range": "± 15579",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5172025,
            "range": "± 177951",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92595,
            "range": "± 9150",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 160729,
            "range": "± 4427",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 211669,
            "range": "± 7379",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 359594,
            "range": "± 7355",
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
          "id": "d2e35009a0c4670198551f3dfa3e06889347113f",
          "message": "Merge pull request #397 from navikt/copilot/jpeg-dimensions-fix-robustness-gap\n\nExtend jpeg_dimensions to cover SOF9–SOF11 arithmetic-coded JPEG markers",
          "timestamp": "2026-08-09T07:55:28+02:00",
          "tree_id": "e1d75969ef4bf2284a38ada1c98306b917bfcdf3",
          "url": "https://github.com/navikt/pdfgenrs/commit/d2e35009a0c4670198551f3dfa3e06889347113f"
        },
        "date": 1786255054575,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 149162,
            "range": "± 2544",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 335429,
            "range": "± 4787",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 4286224,
            "range": "± 53147",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 80458,
            "range": "± 5924",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 112500,
            "range": "± 4790",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 164379,
            "range": "± 7764",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 277083,
            "range": "± 2689",
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
          "id": "dbe3293ca49a9506a48d61954417afad76d76f86",
          "message": "Merge pull request #400 from navikt/copilot/request-tracing-correlation\n\nfix: propagate request ID into Typst compilation span",
          "timestamp": "2026-08-09T08:25:59+02:00",
          "tree_id": "e7b539ea886bdffd62b3ff5ab7462b1c47b3a2a6",
          "url": "https://github.com/navikt/pdfgenrs/commit/dbe3293ca49a9506a48d61954417afad76d76f86"
        },
        "date": 1786256895672,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 235904,
            "range": "± 17236",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 433167,
            "range": "± 4439",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_large_json",
            "value": 5676945,
            "range": "± 148211",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_concurrent",
            "value": 92396,
            "range": "± 6709",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 158820,
            "range": "± 4341",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_jpeg",
            "value": 212454,
            "range": "± 8984",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_svg",
            "value": 367859,
            "range": "± 2792",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}