window.BENCHMARK_DATA = {
  "lastUpdate": 1603639186079,
  "repoUrl": "https://github.com/MoBlaa/irc_rust",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "committer": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "distinct": true,
          "id": "9cf8e9829f1c1e5fa86141a4e464f7ab6d87d643",
          "message": "added personal token to benchmark uploads",
          "timestamp": "2020-09-07T18:49:28+02:00",
          "tree_id": "47c2c603c9b8437059ee1bd5c4048b839a2458dd",
          "url": "https://github.com/MoBlaa/irc_rust/commit/9cf8e9829f1c1e5fa86141a4e464f7ab6d87d643"
        },
        "date": 1599497466782,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench__bench_params_create",
            "value": 283,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_params_iter",
            "value": 2559,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_parse",
            "value": 1168,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_create",
            "value": 1764,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_index",
            "value": 152,
            "range": "± 4",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "committer": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "distinct": true,
          "id": "75a3b69de98c887fab5b33c928bda8c610fa5f57",
          "message": "only running test coverage for irc-rust package",
          "timestamp": "2020-09-07T18:52:58+02:00",
          "tree_id": "f7801e91fc061b0675d01797d6ceefcc4a238d4a",
          "url": "https://github.com/MoBlaa/irc_rust/commit/75a3b69de98c887fab5b33c928bda8c610fa5f57"
        },
        "date": 1599497670262,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench__bench_params_create",
            "value": 240,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_params_iter",
            "value": 2084,
            "range": "± 588",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_parse",
            "value": 1056,
            "range": "± 183",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_create",
            "value": 1497,
            "range": "± 300",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_index",
            "value": 136,
            "range": "± 18",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "mo.blaa@pm.me",
            "name": "MoBlaa",
            "username": "MoBlaa"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "07084d3b163a9f6ff257aac02612778e0fd90eac",
          "message": "Setup Github actions (#1)\n\n* converted project to workspace to extract benchmark code from library to enable building for stable\r\n\r\n* added workflow to build on nightly, stable and perform benchmarks\r\n\r\n* fixed clippy lints\r\n\r\n* reformatted everything with rustfmt\r\n\r\n* reformatted everything with rustfmt\r\n\r\n* using single workflow which includes build, check, fmt, clippy, test and benchmarks\r\n\r\n* only building on push\r\n\r\n* fixed typos and removed pull_request target\r\n\r\n* only building irc-rust\r\n\r\n* using 1.40.0 as MSRV and updated install instructions in readme\r\n\r\n* running github-actions check and clippy only for irc-rust project\r\n\r\n* added missing parallel-finished arg for grcov-finalize\r\n\r\n* only testing on nightly so benchmarks are also counted\r\n\r\n* removed panic_abort flags for tests with coverage as they fail if benchmarks are present too\r\n\r\n* enabling lto for benchmarks\r\n\r\n* added coveralls test coverage\r\n\r\n* added workaround to replace double colons in github-action\r\n\r\n* using underscores instead of spaces for benchmark result workaround\r\n\r\n* added personal token to benchmark uploads\r\n\r\n* only running test coverage for irc-rust package\r\n\r\nCo-authored-by: moblaa <moblaa@pm.me>",
          "timestamp": "2020-09-07T18:58:57+02:00",
          "tree_id": "f7801e91fc061b0675d01797d6ceefcc4a238d4a",
          "url": "https://github.com/MoBlaa/irc_rust/commit/07084d3b163a9f6ff257aac02612778e0fd90eac"
        },
        "date": 1599498005563,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench__bench_params_create",
            "value": 288,
            "range": "± 117",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_params_iter",
            "value": 2516,
            "range": "± 1020",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_parse",
            "value": 1283,
            "range": "± 389",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_create",
            "value": 1714,
            "range": "± 523",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_index",
            "value": 140,
            "range": "± 40",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "committer": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "distinct": true,
          "id": "44400675dc78a1bd750fd51cec1b5ff84e4a86c1",
          "message": "Merge remote-tracking branch 'origin/master'\n\n# Conflicts:\n#\tCargo.toml\n#\tREADME.md\n#\tbench/Cargo.toml\n#\tbench/src/bench.rs\n#\tbench/src/lib.rs\n#\tlib/src/lib.rs",
          "timestamp": "2020-09-07T19:00:54+02:00",
          "tree_id": "f7801e91fc061b0675d01797d6ceefcc4a238d4a",
          "url": "https://github.com/MoBlaa/irc_rust/commit/44400675dc78a1bd750fd51cec1b5ff84e4a86c1"
        },
        "date": 1599498140275,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench__bench_params_create",
            "value": 237,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_params_iter",
            "value": 2023,
            "range": "± 407",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_parse",
            "value": 1067,
            "range": "± 309",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_create",
            "value": 1446,
            "range": "± 382",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_index",
            "value": 135,
            "range": "± 50",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "committer": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "distinct": true,
          "id": "da8adfd783335fee413dab733d87093bbd927bdb",
          "message": "added github-actions workflow badge",
          "timestamp": "2020-09-07T19:08:27+02:00",
          "tree_id": "3db36f1276a2faa4a549a4d4083bfadf75d08113",
          "url": "https://github.com/MoBlaa/irc_rust/commit/da8adfd783335fee413dab733d87093bbd927bdb"
        },
        "date": 1599498570228,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench__bench_params_create",
            "value": 255,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_params_iter",
            "value": 2242,
            "range": "± 851",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_parse",
            "value": 1142,
            "range": "± 449",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_create",
            "value": 1645,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_index",
            "value": 147,
            "range": "± 8",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "committer": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "distinct": true,
          "id": "5f5c6810a56956d6d7676ad5f90521d2ddf4e7b5",
          "message": "updated versions",
          "timestamp": "2020-09-07T19:10:24+02:00",
          "tree_id": "9ba6cf547bf5a98395b7146ac94824abbbaa02c1",
          "url": "https://github.com/MoBlaa/irc_rust/commit/5f5c6810a56956d6d7676ad5f90521d2ddf4e7b5"
        },
        "date": 1599498689158,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench__bench_params_create",
            "value": 257,
            "range": "± 124",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_params_iter",
            "value": 2284,
            "range": "± 287",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_parse",
            "value": 1143,
            "range": "± 128",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_create",
            "value": 1668,
            "range": "± 293",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_index",
            "value": 150,
            "range": "± 10",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "committer": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "distinct": true,
          "id": "5f5c6810a56956d6d7676ad5f90521d2ddf4e7b5",
          "message": "updated versions",
          "timestamp": "2020-09-07T19:10:24+02:00",
          "tree_id": "9ba6cf547bf5a98395b7146ac94824abbbaa02c1",
          "url": "https://github.com/MoBlaa/irc_rust/commit/5f5c6810a56956d6d7676ad5f90521d2ddf4e7b5"
        },
        "date": 1599498843023,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench__bench_params_create",
            "value": 258,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_params_iter",
            "value": 2441,
            "range": "± 407",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_parse",
            "value": 1144,
            "range": "± 327",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_create",
            "value": 1717,
            "range": "± 212",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_index",
            "value": 150,
            "range": "± 28",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "committer": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "distinct": true,
          "id": "bd38417633bc0cefde0e20e314950f93206c774d",
          "message": "updated coverage to only include master coverage and added TOC, Benchmarks and Contributions sections",
          "timestamp": "2020-09-07T19:33:43+02:00",
          "tree_id": "3a81ce4b9e3d406da30b304caf3bad674810c5de",
          "url": "https://github.com/MoBlaa/irc_rust/commit/bd38417633bc0cefde0e20e314950f93206c774d"
        },
        "date": 1599500088893,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench__bench_params_create",
            "value": 232,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_params_iter",
            "value": 1996,
            "range": "± 636",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_parse",
            "value": 989,
            "range": "± 246",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_create",
            "value": 1449,
            "range": "± 305",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_index",
            "value": 140,
            "range": "± 19",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "committer": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "distinct": true,
          "id": "7d0cdc003dd38d8d32bb417b56438b8858fd79e5",
          "message": "fixed some documentation",
          "timestamp": "2020-09-07T19:38:02+02:00",
          "tree_id": "fcb2e51f21574cd01130ef07a255e547275fc2a1",
          "url": "https://github.com/MoBlaa/irc_rust/commit/7d0cdc003dd38d8d32bb417b56438b8858fd79e5"
        },
        "date": 1599500350907,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench__bench_params_create",
            "value": 256,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_params_iter",
            "value": 2290,
            "range": "± 294",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_parse",
            "value": 1125,
            "range": "± 203",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_create",
            "value": 1666,
            "range": "± 106",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_index",
            "value": 150,
            "range": "± 31",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "committer": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "distinct": true,
          "id": "e9928a7020692c4b9c778a5d616f27c7756fb13e",
          "message": "guaranteeing built message is valid through panics",
          "timestamp": "2020-09-09T11:04:12+02:00",
          "tree_id": "d7fab85b12111956d957cff79c74e6bbc9a5fc88",
          "url": "https://github.com/MoBlaa/irc_rust/commit/e9928a7020692c4b9c778a5d616f27c7756fb13e"
        },
        "date": 1599642330773,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench__bench_params_create",
            "value": 228,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_params_iter",
            "value": 2080,
            "range": "± 776",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_parse",
            "value": 1008,
            "range": "± 310",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_create",
            "value": 1870,
            "range": "± 357",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_index",
            "value": 131,
            "range": "± 28",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "committer": {
            "email": "moblaa@pm.me",
            "name": "moblaa"
          },
          "distinct": true,
          "id": "834238b9d4e0d51772f369a129a306e7397650cb",
          "message": "updated version",
          "timestamp": "2020-09-09T11:05:09+02:00",
          "tree_id": "9caa8003d1cd5031ca03cf54f24731992506b984",
          "url": "https://github.com/MoBlaa/irc_rust/commit/834238b9d4e0d51772f369a129a306e7397650cb"
        },
        "date": 1603639184069,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench__bench_params_create",
            "value": 226,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_params_iter",
            "value": 1903,
            "range": "± 462",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_parse",
            "value": 1008,
            "range": "± 163",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_create",
            "value": 1436,
            "range": "± 272",
            "unit": "ns/iter"
          },
          {
            "name": "bench__bench_tag_index",
            "value": 124,
            "range": "± 18",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}