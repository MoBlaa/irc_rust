window.BENCHMARK_DATA = {
  "lastUpdate": 1599497672112,
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
      }
    ]
  }
}