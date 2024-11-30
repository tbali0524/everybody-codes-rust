# Everybody Codes solutions in Rust by TBali

![rust v1.83](https://shields.io/badge/rust-1.83-blue?logo=rust)
![build](https://img.shields.io/github/actions/workflow/status/tbali0524/everybody-codes-rust/qa.yml)
![progress](https://img.shields.io/badge/progress%20⭐-6-yellow)
![license](https://img.shields.io/github/license/tbali0524/everybody-codes-rust)

* [EC website](https://everybody.codes/)
* [Puzzle list](puzzles.md) with topics and my completion status
* Link to this repo on [GitHub](https://github.com/tbali0524/everybody-codes-rust)

## Usage

```sh
# -- setup
# install Rust: https://www.rust-lang.org/tools/install
rustup update stable
# -- info
cargo version
cargo tree
# -- lint
cargo verify-project
cargo fmt
cargo clippy
# -- doc
cargo doc --no-deps --document-private-items --open
# -- test
cargo test
cargo test 2024
cargo test 2024q01
cargo test cli
# in Powershell:
$env:RUST_BACKTRACE=1 ; cargo test
cargo run
cargo run -- 2024
cargo run -- 2024 1
# -- run
cargo build --release
target/release/ec.exe
target/release/ec.exe 2024
target/release/ec.exe 2024 1
# -- shortcut run
./ec.bat
./ec.bat 2024
./ec.bat 2024 1
./ec.bat --help
# -- shortcut qa
./qa.bat
# -- cleanup
cargo clean
```

## Adding a new solution

* add puzzle input in `input/year/ecYYYYqDDpP.txt` and example inputs in `...exX.txt` (separate files for each parts)
* add and edit source in `src/year/ecYYYYqDD.rs`, using the template in `src/ecYYYYqDD.rs`
    * update `pub fn metadata()`, write `solve()`, add unit tests as needed
* edit `src/ecYYYY.rs`:
    * uncomment the `pub mod ecYYYYqDD;` line
    * update the `PUZZLES` list: replace `None` with `Some(...)`
* for a completely new season:
    * edit `src/lib.rs`: add a `pub mod ecYYYY;` line
    * edit `src/ec.rs`: increase `MAX_SEASONS` and add a `Some(...)` item to `PUZZLES`
    * add and update `src/ecYYYY.rs` using the template in `src/ecYYYY.rs`
