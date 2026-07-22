**English** | [日本語版](../ja/CONTRIBUTING.md)

# Contributing Guidelines (CONTRIBUTING.md) - MiSysMon

Thank you for your interest in contributing to the `MiSysMon` project!
This document details guidelines for reporting bugs, proposing features, and submitting pull requests.

---

## 1. Core Principles

Ensure you follow these core development principles:

1. **Lightweight, Safe, and Low-Resource**:
   - Avoid adding unnecessary third-party crates. Keep system polling queries lightweight, and optimize for minimal binary size and memory footprints.
2. **Leverage Shared Library (`common_lib`)**:
   - Reusable low-level utilities (such as double-launch mutex guards or byte formatting logic) should be housed in the adjacent `common_lib` repository, keeping MiSysMon clean.
3. **Sync Multilingual Documents**:
   - When modifying designs, specifications, or adding features, always update files in both `docs/ja/` and `docs/en/` directories.

---

## 2. Workspace Setup

1. **Clone `common_lib` in the parent directory (side-by-side)**:
   ```bash
   git clone https://github.com/tkshnkgwr/common_lib.git
   ```
2. **Clone `MiSysMon`**:
   ```bash
   git clone https://github.com/tkshnkgwr/MiSysMon.git
   cd MiSysMon
   ```
3. **Run the App**:
   ```bash
   cargo run
   ```

---

## 3. Commit and Pull Request Guidelines

### Commit Message Conventions
Use the Conventional Commits format:

- `feat:` New features
- `fix:` Bug fixes
- `docs:` Document updates
- `refactor:` Code refactoring
- `perf:` Performance optimizations
- `test:` Unit/integration tests
- `chore:` Maintenance and config adjustments

### PR Checklist
Before opening a pull request, ensure all of the following commands pass successfully:

- [ ] `cargo test` (unit tests pass)
- [ ] `cargo clippy --all-targets -- -D warnings` (clippy warning-free)
- [ ] `cargo fmt --check` (format check passes)
- [ ] `cargo doc --no-deps --document-private-items` (rustdoc compiles without errors)
