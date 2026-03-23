# Code Quality Fixes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix all correctness bugs, silent failures, dead code, and structural issues found in the QA analysis, and split the single-file binary into focused modules.

**Architecture:** Split `src/main.rs` into three focused modules (`duration.rs`, `time.rs`, `main.rs`), fix the regex correctness bug, thread errors through `parse_args → main` so bad input exits non-zero, and optionally eliminate the `dateparser` heavyweight dependency in favor of a direct chrono-based parser.

**Tech Stack:** Rust 2021 edition, `regex`, `chrono` (direct), `std::sync::OnceLock` (stable, no new deps).

---

## File Map

| File                 | Status   | Responsibility                                          |
| -------------------- | -------- | ------------------------------------------------------- |
| `src/duration.rs`    | Create   | `parse_duration`, `DurationError`, unit tests           |
| `src/time.rs`        | Create   | `parse_time`, unit tests                                |
| `src/main.rs`        | Modify   | `main`, `parse_args` (returning `Result`), mod declares |
| `Cargo.toml`         | Modify   | Remove `dateparser`, keep `regex` (Task 6 only)         |

---

### Task 1: Extract `duration.rs` module

**Files:**
- Create: `src/duration.rs`
- Modify: `src/main.rs`

- [x] **Step 1: Create `src/duration.rs`** — move `DurationError`, `parse_duration`, and all `parse_duration` tests verbatim from `main.rs`.

```rust
// src/duration.rs
use regex::Regex;
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub struct DurationError;

pub fn parse_duration(s: &str) -> Result<Duration, DurationError> {
    let re = Regex::new(r"(\d+)(ms|s|m|h)?$").unwrap();
    match re.captures(s) {
        Some(caps) => {
            let value = caps.get(1).map_or("", |m| m.as_str());
            let num = value.parse::<u64>().unwrap();
            let unit = &caps.get(2).map_or("", |m| m.as_str()).to_lowercase();
            let duration = match unit.as_str() {
                "ns" => Duration::from_nanos(num),
                "ms" => Duration::from_millis(num),
                "s"  => Duration::from_secs(num),
                "m"  => Duration::from_secs(num * 60),
                "h"  => Duration::from_secs(num * 3600),
                _    => Duration::from_millis(num),
            };
            Ok(duration)
        }
        None => Err(DurationError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // ... all existing parse_duration tests go here unchanged
}
```

- [x] **Step 2: Create `src/time.rs`** — move `parse_time` and all `parse_time` tests verbatim.

```rust
// src/time.rs
use std::time::{Duration, SystemTime};

pub fn parse_time(input: &str) -> Result<Duration, String> {
    match dateparser::parse(input) {
        Ok(parsed) => {
            let now = SystemTime::now();
            let current_time = now
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|_| "SystemTime before UNIX EPOCH!".to_string())?
                .as_micros();
            let target = parsed.timestamp_micros() as u128;
            if target > current_time {
                let delta = target - current_time;
                return Ok(Duration::from_micros(delta as u64));
            }
            Ok(Duration::from_micros(0))
        }
        Err(_) => Err("Failed to parse".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // ... all existing parse_time tests go here unchanged
}
```

- [x] **Step 3: Update `src/main.rs`** — declare modules, remove moved code, keep `parse_args` and `main`.

```rust
// src/main.rs
mod duration;
mod time;

use duration::parse_duration;
use time::parse_time;
use std::time::Duration;

fn parse_args() -> Duration {
    // unchanged for now
}

fn main() {
    let dur = parse_args();
    std::thread::sleep(dur);
    std::process::exit(0);
}
```

- [x] **Step 4: Run tests**

```bash
cargo test --verbose
```
Expected: all existing tests pass.

- [x] **Step 5: Commit**

```bash
git add src/duration.rs src/time.rs src/main.rs
git commit -m "refactor: split main.rs into duration and time modules"
```

---

### Task 2: Fix regex correctness (`^` anchor + `ns` unit)

**Files:**
- Modify: `src/duration.rs`

The regex `(\d+)(ms|s|m|h)?$` has two bugs:
1. No `^` anchor — `abc123s` matches and sleeps 123s silently.
2. `ns` is in the match arm but not in the regex alternation — the arm is unreachable.

- [x] **Step 1: Update the "known bug" tests to reflect the corrected behavior**

In `src/duration.rs` tests, change the two "known behavior" tests:

```rust
#[test]
fn test_parse_duration_ns() {
    assert_eq!(parse_duration("100ns"), Ok(Duration::from_nanos(100)));
    assert_eq!(parse_duration("0ns"), Ok(Duration::from_nanos(0)));
}

#[test]
fn test_parse_duration_rejects_leading_junk() {
    assert_eq!(parse_duration("abc123s"), Err(DurationError));
    assert_eq!(parse_duration("xyz500ms"), Err(DurationError));
}
```

- [x] **Step 2: Run tests — verify they fail**

```bash
cargo test test_parse_duration_ns test_parse_duration_rejects_leading_junk -- --nocapture
```
Expected: both FAIL.

- [x] **Step 3: Fix the regex in `src/duration.rs:8`**

```rust
// Before:
let re = Regex::new(r"(\d+)(ms|s|m|h)?$").unwrap();

// After:
let re = Regex::new(r"^(\d+)(ns|ms|s|m|h)?$").unwrap();
```

- [x] **Step 4: Run all tests**

```bash
cargo test --verbose
```
Expected: all pass. The old "known bug" tests are now gone; the new ones pass.

- [x] **Step 5: Commit**

```bash
git add src/duration.rs
git commit -m "fix: add ^ anchor to duration regex and support ns unit"
```

---

### Task 3: Static regex with `OnceLock`

**Files:**
- Modify: `src/duration.rs`

Currently the regex is compiled on every `parse_duration` call. Use `std::sync::OnceLock` (stable since Rust 1.70, no new dependencies).

- [x] **Step 1: Replace inline `Regex::new` with a static**

```rust
// src/duration.rs
use regex::Regex;
use std::sync::OnceLock;
use std::time::Duration;

static DURATION_RE: OnceLock<Regex> = OnceLock::new();

fn duration_regex() -> &'static Regex {
    DURATION_RE.get_or_init(|| {
        Regex::new(r"^(\d+)(ns|ms|s|m|h)?$").unwrap()
    })
}

pub fn parse_duration(s: &str) -> Result<Duration, DurationError> {
    let re = duration_regex();
    match re.captures(s) {
        // ... rest unchanged
    }
}
```

- [x] **Step 2: Run all tests**

```bash
cargo test --verbose
```
Expected: all pass.

- [x] **Step 3: Commit**

```bash
git add src/duration.rs
git commit -m "perf: compile duration regex once via OnceLock"
```

---

### Task 4: Proper error propagation — exit non-zero on bad input

**Files:**
- Modify: `src/main.rs`

Currently `parse_args()` returns `Duration` and swallows all errors by returning zero duration. `hang badinput` exits 0 silently — this breaks scripts. Fix: return `Result`, print errors to stderr, exit 1.

- [x] **Step 1: Change `parse_args` signature and body in `src/main.rs`**

```rust
fn parse_args() -> Result<Duration, String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Ok(Duration::from_secs(1));
    }

    let input = &args[1];
    if input.contains(':') {
        return parse_time(input);
    }

    parse_duration(input).map_err(|_| format!("invalid duration: '{}'", input))
}
```

- [x] **Step 2: Update `main` to handle the Result**

```rust
fn main() {
    match parse_args() {
        Ok(dur) => std::thread::sleep(dur),
        Err(msg) => {
            eprintln!("hang: {}", msg);
            std::process::exit(1);
        }
    }
}
```

- [x] **Step 3: Run all tests**

```bash
cargo test --verbose
```
Expected: all pass (existing tests target `parse_duration`/`parse_time` directly, not `parse_args`).

- [x] **Step 4: Manual smoke test**

```bash
cargo build
./target/debug/hang badinput; echo "exit: $?"
```
Expected: prints `hang: invalid duration: 'badinput'` to stderr, exits 1.

```bash
./target/debug/hang 1s; echo "exit: $?"
```
Expected: sleeps 1s, exits 0.

- [x] **Step 5: Commit**

```bash
git add src/main.rs
git commit -m "fix: propagate parse errors to stderr and exit 1 on bad input"
```

---

### Task 5: Remove dead code and housekeeping

**Files:**
- Modify: `src/main.rs`
- Modify: `src/duration.rs`

- [x] **Step 1: Remove commented-out lines**

In `src/duration.rs` (was `main.rs`), delete:
```rust
// _ => return Err(DurationError),
```

In `src/main.rs`, delete:
```rust
// print!("duration {:?}", dur);
```

- [x] **Step 2: Run tests and clippy**

```bash
cargo clippy -- -D warnings && cargo test
```
Expected: no warnings, all tests pass.

- [x] **Step 3: Commit**

```bash
git add src/main.rs src/duration.rs
git commit -m "chore: remove commented-out dead code"
```

---

### Task 6 (Strategic): Replace `dateparser` with a focused `HH:MM:SS` parser

**Files:**
- Modify: `src/time.rs`
- Modify: `Cargo.toml`

`dateparser` brings in `anyhow`, `chrono`, and `core-foundation-sys` for what the CLAUDE.md documents as: _"target times in `HH:MM:SS` format"_. We can replace it with a direct `chrono`-based parser (chrono is already a transitive dep) and a hand-written `HH:MM:SS` parser, eliminating `dateparser` entirely.

The `:` heuristic in `parse_args` is also fragile — make it route only on strings that match `HH:MM:SS` exactly.

- [ ] **Step 1: Write failing tests for the new HH:MM:SS parser in `src/time.rs`**

```rust
#[test]
fn test_parse_time_hms_format() {
    // These all contain ':' and should route to parse_time
    let result = parse_time("23:59:59");
    assert!(result.is_ok());

    let result = parse_time("00:00:00");
    assert!(result.is_ok());
}

#[test]
fn test_parse_time_invalid_hms() {
    assert!(parse_time("25:00:00").is_err()); // invalid hour
    assert!(parse_time("12:60:00").is_err()); // invalid minute
    assert!(parse_time("12:00:60").is_err()); // invalid second
    assert!(parse_time("notadate").is_err());
}
```

- [ ] **Step 2: Run — verify new tests fail or pass unexpectedly**

```bash
cargo test test_parse_time -- --nocapture
```

- [ ] **Step 3: Rewrite `src/time.rs`** to use `chrono` directly without `dateparser`

```rust
// src/time.rs
use chrono::{Local, NaiveTime, Timelike};
use std::time::Duration;

pub fn parse_time(input: &str) -> Result<Duration, String> {
    let target = NaiveTime::parse_from_str(input, "%H:%M:%S")
        .map_err(|_| format!("invalid time: '{}'", input))?;

    let now = Local::now().time();
    let target_secs = target.num_seconds_from_midnight() as i64;
    let now_secs = now.num_seconds_from_midnight() as i64;
    let delta = target_secs - now_secs;

    if delta > 0 {
        Ok(Duration::from_secs(delta as u64))
    } else {
        Ok(Duration::from_secs(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // ... updated tests (note: ISO 8601 tests are removed since dateparser is gone)
}
```

- [ ] **Step 4: Update `src/main.rs`** — tighten the dispatch heuristic to only match `HH:MM:SS`

```rust
fn is_time_format(s: &str) -> bool {
    // Accept only HH:MM:SS — exactly two colons, all digits in each segment
    let parts: Vec<&str> = s.split(':').collect();
    parts.len() == 3 && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn parse_args() -> Result<Duration, String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Ok(Duration::from_secs(1));
    }
    let input = &args[1];
    if is_time_format(input) {
        return parse_time(input);
    }
    parse_duration(input).map_err(|_| format!("invalid duration: '{}'", input))
}
```

- [ ] **Step 5: Remove `dateparser` from `Cargo.toml`**

```toml
[dependencies]
chrono = "0.4"
regex  = "1.10.5"
```

- [ ] **Step 6: Run all tests and clippy**

```bash
cargo clippy -- -D warnings && cargo test --verbose
```
Expected: all pass, no warnings.

- [ ] **Step 7: Manual smoke test**

```bash
cargo build
./target/debug/hang 23:59:59; echo "exit: $?"   # should sleep ~0-86399s depending on time of day
./target/debug/hang 00:00:00; echo "exit: $?"   # past time, sleeps 0, exits 0
./target/debug/hang 2999-01-01; echo "exit: $?" # invalid format, exits 1
```

- [ ] **Step 8: Commit**

```bash
git add src/time.rs src/main.rs Cargo.toml Cargo.lock
git commit -m "refactor: replace dateparser with direct chrono HH:MM:SS parser"
```

---

## Summary of Changes

| Task | Impact              | Risk  |
| ---- | ------------------- | ----- |
| 1    | Module split        | Low   |
| 2    | Correctness fix     | Low   |
| 3    | Performance/clarity | Low   |
| 4    | Correctness + UX    | Low   |
| 5    | Housekeeping        | None  |
| 6    | Dep reduction       | Medium (removes dateparser's flexible parsing) |

Tasks 1–5 are safe and independent. Task 6 intentionally narrows the accepted time format to exactly `HH:MM:SS` — confirm this matches the intended scope before executing.
