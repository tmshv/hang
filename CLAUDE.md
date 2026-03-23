# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**hang** is a Rust CLI tool that pauses execution for a specified duration or until a specific time. It accepts duration strings (e.g., `5s`, `100ms`, `2h`) or target times in `HH:MM:SS` format. With no arguments, it sleeps for 1 second.

## Build & Development Commands

```bash
cargo build              # Build the project
cargo build --verbose    # Build with verbose output (used in CI)
cargo run -- 5s          # Run with a duration argument
cargo test               # Run all tests
cargo test --verbose     # Run tests with verbose output (used in CI)
cargo test <test_name>   # Run a single test by name
```

## Architecture

Three-module Rust binary:

1. **`src/duration.rs`** — `parse_duration(s)` — Regex-based parser for short duration syntax (`5s`, `100ms`, `2h`, `10m`, `100ns`). Bare numbers without a unit suffix are treated as milliseconds. Regex is compiled once via `OnceLock`.
2. **`src/time.rs`** — `parse_time(input)` — Parses `HH:MM:SS` using `chrono::NaiveTime::parse_from_str`. Computes subsecond-accurate delta from now; past times resolve to zero.
3. **`src/main.rs`** — `parse_args()` routes to `parse_time` if input is exactly three colon-separated all-digit non-empty segments (e.g. `10:20:30`), otherwise to `parse_duration`. Errors are printed to stderr and exit with code 1.

## Dependencies

- `regex` — Duration string pattern matching
- `chrono` — Time parsing and local clock access for `HH:MM:SS` target-time calculation

## Code Quality

Always verify formatting and lints before committing:

```bash
cargo fmt --check   # Verify formatting (run cargo fmt to fix)
cargo clippy        # Check for lints
```

## CI

GitHub Actions workflow (`.github/workflows/rust.yml`) runs `cargo build` and `cargo test` on pushes and PRs to `master`.
