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

Single-file Rust binary (`src/main.rs`) with three parsing paths:

1. **`parse_duration(s)`** — Regex-based parser for short duration syntax (`5s`, `100ms`, `2h`, `10m`). Bare numbers without a unit suffix are treated as milliseconds.
2. **`parse_time(input)`** — Uses the `dateparser` crate to parse absolute timestamps, then computes the delta from now. Past times resolve to zero duration.
3. **`parse_args()`** — Entry point for argument handling. Routes to `parse_time` if the input contains `:`, otherwise to `parse_duration`.

## Dependencies

- `regex` — Duration string pattern matching
- `dateparser` — Flexible date/time string parsing

## CI

GitHub Actions workflow (`.github/workflows/rust.yml`) runs `cargo build` and `cargo test` on pushes and PRs to `master`.
