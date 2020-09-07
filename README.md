# irc-rust 

![Build Workflow](https://github.com/MoBlaa/irc_rust/workflows/Rust%20Overall%20workflow/badge.svg?branch=master)
[![https://docs.rs/irc-rust/badge.svg](https://docs.rs/irc-rust/badge.svg)](https://docs.rs/irc-rust)
[![crates.io](https://img.shields.io/crates/v/irc-rust.svg)](https://crates.io/crates/irc-rust)
[![Coverage Status](https://coveralls.io/repos/github/MoBlaa/irc_rust/badge.svg?branch=github-actions)](https://coveralls.io/github/MoBlaa/irc_rust?branch=github-actions)

IRC Helper easing the access and creation of IRC Messages. Minimum supported rust version (MRSV) is **1.40.0**.

Github-actions runs `build`, `check`, `fmt`, `clippy` and `test` against the latest stable, nightly and 1.40.0 rust toolchains.

## Installation

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
irc-rust = "0.3"
```
- Or use [`cargo edit`](https://github.com/killercup/cargo-edit) to get the latest every time:
```shell script
cargo install cargo-edit
cargo add irc-rust # In your project root
```

## Basic Usage

```rust
use irc_rust::Message;

fn main() {
    let message = Message::from("@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
    
    assert_eq!(message.to_string(), "@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
}
```
