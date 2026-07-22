**English** | [日本語版](../ja/TESTING.md)

# Testing Guidelines & Execution (TESTING.md) - MiSysMon

This document outlines the testing policies, verification targets, and execution procedures for quality assurance in the `MiSysMon` project.

---

## 1. Test Overview

`MiSysMon` is a lightweight system monitor built with Rust, egui, eframe, sysinfo, and other crates. We perform targeted testing and verification to maintain its reliability and stability.

Key Testing Viewpoints:
- **Telemetry Precision**: Correct calculations for CPU usage/temperature, memory footprint, network bandwidth, disk space, and disk I/O rates.
- **Double-Launch Prevention**: Lock guards utilizing Named Mutex via the shared crate `common_lib`.
- **Coordinate Persistence & Validation**: Settings serialization (app.ron) and out-of-monitor boundary validation at startup via Win32 API `MonitorFromPoint`.
- **UI Layout & Operations**: Absolute coordinates dragging via the handle, 15px border auto-snap, and dynamic window width adjustments on toggles.

---

## 2. Running Tests

Procedures for executing local checks and tests:

### Executing Unit Tests
```bash
cargo test
```

### Verification Checklist Commands
Before submitting PRs or finalizing tasks, ensure that all of the following validation commands pass successfully with zero warnings/errors:

```bash
# 1. Run Unit Tests
cargo test

# 2. Static Analysis (Clippy)
cargo clippy --all-targets -- -D warnings

# 3. Code Formatting Check
cargo fmt --check

# 4. Rustdoc Build Validation
cargo doc --no-deps --document-private-items
```

---

## 3. Guidelines for Writing Tests

1. **In-Module Unit Testing**:
   - Implement unit tests (like byte formatting) within `src/main.rs` to verify functional correctness.
2. **Crash Prevention Checks**:
   - Test fallback paths to ensure that errors from sensor polling or OS-specific operations do not panic or crash the application.
