window.BENCHMARK_DATA = {
  "lastUpdate": 1785485615922,
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
          "id": "b0285638aaf4343bd62eb8b011327cf54d47f028",
          "message": "Merge pull request #362 from navikt/copilot/html-to-pdf\n\nRemove stale html_to_pdf benchmark from criterion report",
          "timestamp": "2026-07-24T18:03:47+02:00",
          "tree_id": "210dd630584778d65fe594af630f5a9af45bf305",
          "url": "https://github.com/navikt/pdfgenrs/commit/b0285638aaf4343bd62eb8b011327cf54d47f028"
        },
        "date": 1784909170449,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 233723,
            "range": "± 18117",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 428290,
            "range": "± 2039",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 158323,
            "range": "± 4664",
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
          "id": "7e9d73b53e833891664ffb83b46139738364e7a4",
          "message": "Merge pull request #363 from navikt/copilot/testutil-duplicates-reduction\n\nrefactor: eliminate duplication between make_state and make_state_with_body_limit",
          "timestamp": "2026-07-29T09:52:29+02:00",
          "tree_id": "a80d9734cf6671cda186df7678a34f1f511dc648",
          "url": "https://github.com/navikt/pdfgenrs/commit/7e9d73b53e833891664ffb83b46139738364e7a4"
        },
        "date": 1785311658860,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 216239,
            "range": "± 11855",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 400748,
            "range": "± 8198",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 134989,
            "range": "± 3245",
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
          "id": "9787e370ec68177266eb1f28caa2f4d8a87f3a25",
          "message": "Merge pull request #364 from navikt/copilot/optimize-template-storage\n\nrefactor: accept &str instead of String in template compilation APIs to avoid per-request allocation",
          "timestamp": "2026-07-29T10:45:26+02:00",
          "tree_id": "e7c93eae2c55a4c33605b27c48eeb7829510eeb0",
          "url": "https://github.com/navikt/pdfgenrs/commit/9787e370ec68177266eb1f28caa2f4d8a87f3a25"
        },
        "date": 1785314858826,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 231733,
            "range": "± 25131",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 420835,
            "range": "± 8122",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 148109,
            "range": "± 4328",
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
          "id": "377aa07f155462b039105434720b620644b6a7de",
          "message": "Merge pull request #366 from navikt/copilot/add-compilation-duration-histogram-metric\n\nAdd compilation duration histogram metric",
          "timestamp": "2026-07-30T09:47:39+02:00",
          "tree_id": "0e87b97ccbe776bb26301797c5f998adc0df708f",
          "url": "https://github.com/navikt/pdfgenrs/commit/377aa07f155462b039105434720b620644b6a7de"
        },
        "date": 1785398138946,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 225891,
            "range": "± 2116",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 417577,
            "range": "± 15582",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 148680,
            "range": "± 5278",
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
          "id": "d9d0824cbf7a451a4a3b4af884fa0276184310c6",
          "message": "Merge pull request #367 from navikt/copilot/missing-tests-for-image-and-template-issues\n\nAdd missing tests for corrupted images, large templates, concurrent compilation, font loading failures, and exact-boundary body limits",
          "timestamp": "2026-07-30T09:49:41+02:00",
          "tree_id": "0dc21a11894918bc5e065fce3a54d70db9cb6c0c",
          "url": "https://github.com/navikt/pdfgenrs/commit/d9d0824cbf7a451a4a3b4af884fa0276184310c6"
        },
        "date": 1785398252676,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 179769,
            "range": "± 5370",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 399831,
            "range": "± 3075",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 118170,
            "range": "± 7111",
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
          "id": "80108e82ad84b58a2cba78dc38db9a473beee9be",
          "message": "Merge pull request #368 from navikt/copilot/replace-hashmap-value-with-arc-value\n\nReplace HashMap<..., Value> with HashMap<..., Arc<Value>>",
          "timestamp": "2026-07-30T10:23:45+02:00",
          "tree_id": "1691b3a0c506043bdbf321df1fbd46aa423f668c",
          "url": "https://github.com/navikt/pdfgenrs/commit/80108e82ad84b58a2cba78dc38db9a473beee9be"
        },
        "date": 1785399920469,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 225509,
            "range": "± 14598",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 419588,
            "range": "± 5860",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 147853,
            "range": "± 4909",
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
          "id": "e41c3676177e4951b23850b6d11e32765d4783f6",
          "message": "Merge pull request #371 from navikt/copilot/timeout-behavior-fix\n\nfix: release semaphore permit on timeout and abort orphaned spawn_blocking task",
          "timestamp": "2026-07-30T18:23:35+02:00",
          "tree_id": "ef3c1d17d9bf04f2ee0ca401fc30cb1a379d73d7",
          "url": "https://github.com/navikt/pdfgenrs/commit/e41c3676177e4951b23850b6d11e32765d4783f6"
        },
        "date": 1785428711263,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 229290,
            "range": "± 13154",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 425966,
            "range": "± 10178",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 150841,
            "range": "± 4362",
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
          "id": "1c53ff0ba36ed99ff2b727f3e0f65dffddfc1eb9",
          "message": "Merge pull request #373 from navikt/copilot/add-must-use-annotation-html-typst-to-html\n\nAdd #[must_use] to html::typst_to_html",
          "timestamp": "2026-07-31T09:11:33+02:00",
          "tree_id": "609508fce762742dfc6d8d2a18e424ad37a52ddc",
          "url": "https://github.com/navikt/pdfgenrs/commit/1c53ff0ba36ed99ff2b727f3e0f65dffddfc1eb9"
        },
        "date": 1785481976232,
        "tool": "cargo",
        "benches": [
          {
            "name": "typst_to_pdf_simple",
            "value": 231741,
            "range": "± 1508",
            "unit": "ns/iter"
          },
          {
            "name": "typst_to_pdf_with_data",
            "value": 432855,
            "range": "± 27153",
            "unit": "ns/iter"
          },
          {
            "name": "image_to_pdf_png",
            "value": 156069,
            "range": "± 3620",
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
      }
    ]
  }
}