# AGENTS.md

## Project Overview
This is an educational Rust project consisting of independent crates, each corresponding to a chapter from "The Rust Programming Language" book. Each `chX-*` directory contains a separate binary or library crate demonstrating specific Rust concepts.

## Architecture
- **Structure**: Root contains multiple chapter directories (e.g., `ch2-programming-guessing-game`, `ch12-io-project-cli/minigrep`).
- **Independence**: Each crate is standalone with its own `Cargo.toml` and `src/` directory.
- **Purpose**: Chapters build sequentially; later chapters depend on concepts from earlier ones but are not code-dependent.

## Key Patterns
- **CLI Tools**: Use `std::env::args()` for argument parsing. Define a `Config` struct with `build()` method returning `Result<Config, &'static str>`. Implement a `run(config: Config) -> Result<(), Box<dyn Error>>` function called from `main()`. Example: `ch12-io-project-cli/minigrep/src/main.rs`.
- **Library with Tests**: Place public functions in `src/lib.rs`, tests in `#[cfg(test)] mod tests`. Use `super::*` in tests. Example: `ch10-types-traits-lifetime/src/lib.rs`.
- **Error Handling**: Use `unwrap_or_else` in `main()` for config and run errors, printing to stderr and exiting with code 1.
- **Dependencies**: Specify exact versions in `Cargo.toml` (e.g., `rand = "=0.8.5"` in `ch2-programming-guessing-game/Cargo.toml`).

## Workflows
- **Build**: `cd` to chapter directory, run `cargo build`.
- **Run**: `cargo run` in chapter directory.
- **Test**: `cargo test` in chapter directory.
- **Debug**: Use `println!` for output; no custom debug setup.

## Conventions
- **Naming**: Snake_case for functions and variables (e.g., `add_one`, `file_path`).
- **Structs**: Define with `pub` fields if needed; implement methods in `impl` blocks.
- **Lifetimes**: Use explicit lifetimes in function signatures when returning references (e.g., `search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>` in `ch12-io-project-cli/minigrep/src/lib.rs`).

## Dependencies
- Minimal external crates; only when required for chapter examples (e.g., `rand` for random numbers).
- Edition: 2024 across all crates.

## Examples
- Simple function: `fn add_one(mut x: f32) -> f32 { x += 1.0; x }` in `ch14-cargo-and-crates/src/main.rs`.
- Iterator usage: `contents.lines().filter(|line| line.contains(query)).collect()` in `ch12-io-project-cli/minigrep/src/lib.rs`.
