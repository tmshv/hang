---
name: bump-version
description: Use when user asks to release, bump, or publish a new version (major, minor, or patch) of this Rust project
---

# Bump Version

Automates releasing a new version of the `hang` Rust CLI project.

## Step 1: Determine bump type

Ask the user which version component to bump if not specified:
- **major** — breaking change (e.g. `1.2.3` → `2.0.0`)
- **minor** — new feature (e.g. `1.2.3` → `1.3.0`)
- **patch** — bug fix (e.g. `1.2.3` → `1.2.4`)

## Step 2: Verify before bumping

Read `Cargo.toml` to get the current version. Compute the new version and **confirm with the user** before making any changes:

> Current version: `0.1.0`. Bumping minor → `0.2.0`. Proceed?

## Step 3: Execute release

Only after user confirms:

1. **Update `Cargo.toml`** with new version string
2. **Sync `Cargo.lock`** — run `cargo build`
3. **Verify quality**:
   ```
   cargo fmt --check
   cargo clippy
   cargo test
   ```
   Fix any issues before continuing.
4. **Update `README.md`** — set `--tag vX.Y.Z` in the install section to the new version
5. **Commit**:
   ```
   git add Cargo.toml Cargo.lock README.md
   git commit -m "Bump version to vX.Y.Z"
   ```
6. **Tag**:
   ```
   git tag vX.Y.Z
   ```
7. **Push**:
   ```
   git push && git push --tags
   ```
8. **Create GitHub release**:
   ```
   gh release create vX.Y.Z --generate-notes
   ```
