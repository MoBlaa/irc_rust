# irc-rust 

![Build Workflow](https://github.com/MoBlaa/irc_rust/workflows/Rust%20Overall%20workflow/badge.svg?branch=master)
[![https://docs.rs/irc-rust/badge.svg](https://docs.rs/irc-rust/badge.svg)](https://docs.rs/irc-rust)
[![crates.io](https://img.shields.io/crates/v/irc-rust.svg)](https://crates.io/crates/irc-rust)
[![Coverage Status](https://coveralls.io/repos/github/MoBlaa/irc_rust/badge.svg?branch=github-actions)](https://coveralls.io/github/MoBlaa/irc_rust?branch=master)

IRC Helper easing the access and creation of IRC Messages. Minimum supported rust version (MRSV) is **1.40.0**.

Github-actions runs `build`, `check`, `fmt`, `clippy` and `test` against the latest stable, nightly and 1.40.0 rust toolchains.

# Table of Contents

   * [Installation](#installation)
   * [Basic Usage](#basic-usage)
   * [Benchmarks](#benchmarks)
   * [Contributions](#contributions)

# Installation

Requirements:

- [`Install nightly Rust toolchain`](https://www.rust-lang.org/tools/install): 
```shell script
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
.rustup.rs | sh
```

Installation: 

- Add to `Cargo.toml`:
```toml
[dependencies]
irc-rust = "0.4"
```
- Or use [`cargo edit`](https://github.com/killercup/cargo-edit) to get the latest every time:
```shell script
cargo install cargo-edit
cargo add irc-rust # In your project root
```

# Basic Usage

```rust
use irc_rust::Message;

fn main() {
    let message = Message::from("@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
    
    assert_eq!(message.to_string(), "@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
}
```

# Benchmarks

Benchmark results are uploaded to github-pages automatically on push to master: [https://moblaa.github.io/irc_rust/dev/bench/](https://moblaa.github.io/irc_rust/dev/bench/)

The code for the benchmarks is located in the [bench](bench) subdirectory.

# Contributions

If you have any suggestion may it be refactorings of the code or benchmarks, have a feature request or something else, feel free to open an issue or pull request so we can discuss.

This is my first open source library, so I'm also open for advise on how to manage this repository.
