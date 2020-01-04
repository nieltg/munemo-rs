# munemo-rs

[![Build Status](https://travis-ci.com/nieltg/munemo-rs.svg?branch=master)](https://travis-ci.com/nieltg/munemo-rs)
[![Latest Version](https://img.shields.io/crates/v/munemo-rs.svg)](https://crates.io/crates/munemo-rs)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/munemo-rs)

Turn an integer into a Japan-ish word which is more easy to remember, or vice-versa.

Example of numbers and their encoded value:

|  Number  | Encoded Value |
|:--------:|:-------------:|
|    -1    |      xabi     |
|   2248   |      gume     |
|   2592   |      hayu     |
|  475349  |     munemo    |
| 94714682 |   yoshimitsu  |

**munemo** is my first crate and is one of the means for me to study [Rust](https://www.rust-lang.org).

## Getting Started

To use or develop this crate, you need [Rust](https://www.rust-lang.org/tools/install).

### Usage

Add [this crate](https://crates.io/crates/munemo-rs) under `[dependency]` section in your `Cargo.toml` file.

Usage of this library is described on [the documentation](https://docs.rs/munemo-rs).

### Hacking

This library is built using Test-Driven Development pattern.

1. Write the test.
2. Ensure the test fails using `cargo test`.
3. Write the implementation.
4. Ensure the failing test has been suceeded using `cargo test`.
5. Make a Git commit.

Additional steps if you need to refactor:

1. Refactor the code.
2. Make sure the tests success using `cargo test`.
3. Make a Git commit.

To build this library, you can use `cargo build`.

Additional `cargo` commands can be discovered in [the documentation](https://doc.rust-lang.org/cargo/guide).

## License

[MIT](LICENSE).

## References

Creation of this crate was inspired from [munemo Ruby gem](https://rubygems.org/gems/munemo) by [John Mettraux](https://github.com/jmettraux).

In additional, here are resources I learnt while making this crate:
- [The Rust Programming Language](https://doc.rust-lang.org/book).
  - [Defining Modules to Control Scope and Privacy](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html).
  - [Separating Modules into Different Files](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html).
  - [How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html).
  - [Publishing a Crate to Crates.io](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html).
- [Rust API Guidelines Checklist](https://rust-lang.github.io/api-guidelines/checklist.html).
- [Crate rand](https://docs.rs/rand/0.7.2/rand).
- [About Docs.rs](https://docs.rs/about).
