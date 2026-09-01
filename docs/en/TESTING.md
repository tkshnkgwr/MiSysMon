**English** | [日本語版](../ja/TESTING.md)

# Testing Guidelines & Execution (TESTING.md) - MiSysMon

This document outlines the testing policies, verification targets, modular test suites, and execution procedures for quality assurance in the `MiSysMon` project.

---

## 1. Test Overview

`MiSysMon` is a lightweight system monitor built with Rust, egui, eframe, sysinfo, and other crates. We perform targeted testing and verification to maintain high reliability and stability.

Key Testing Viewpoints:
- **Configuration Persistence & Backward Compatibility (`config`)**: Complete serialization/deserialization roundtrips via `serde_json`, and safe default fallbacks for missing or partial configuration fields.
- **Window Snapping & Boundary Checks (`win32`)**: Verification of 15px edge auto-snapping (left, right, top, bottom, and corners) and non-snapping behavior outside the threshold.
- **UI Layout & Dynamic Width (`ui`)**: Verification of dynamic window width calculation across full display, all-disabled (200px minimum clamp), and custom item selection.
- **Metrics Management & Log Succession (`monitor`)**: Bounded FIFO sliding behavior for the 30-item sparkline history queue, multi-generation log file rotation (`.1`, `.2`), and saturating subtraction underflow protection.
- **Double-Launch Prevention**: Mutex lock guards utilizing Named Mutex via the shared crate `common_lib`.

---

## 2. Modular Unit Test Suite

The 15 active unit tests are categorized across modules as follows:

| Module      | Test Function Name                                              | Verification Details                                                   |
| :---------- | :-------------------------------------------------------------- | :--------------------------------------------------------------------- |
| `config`    | `test_config_default_values`                                    | Consistency of default settings (all enabled, 1.0s interval, opacity)  |
| `config`    | `test_config_json_serialization_roundtrip`                      | Full roundtrip fidelity of JSON serialization and deserialization      |
| `config`    | `test_config_json_partial_fallback`                             | Safe fallback to defaults when partial fields are supplied             |
| `win32`     | `test_apply_snap_left_edge`                                     | Snapping to left work area border within 15px and negative coordinates |
| `win32`     | `test_apply_snap_right_edge`                                    | Snapping to right work area border within 15px                          |
| `win32`     | `test_apply_snap_top_edge`                                      | Snapping to top work area border within 15px                            |
| `win32`     | `test_apply_snap_bottom_edge`                                   | Snapping to bottom work area border within 15px                         |
| `win32`     | `test_apply_snap_no_snap_when_outside_threshold`                | No snapping when outside the 15px threshold                            |
| `win32`     | `test_apply_snap_corner_snap`                                   | Simultaneous 2-axis snapping at corner boundaries                      |
| `ui`        | `test_calculate_window_width_all_enabled`                       | Dynamic width calculation with all elements enabled (1210px)           |
| `ui`        | `test_calculate_window_width_all_disabled_minimum_clamping`     | Minimum window width clamp when all elements are hidden (200px)        |
| `ui`        | `test_calculate_window_width_custom_selection`                  | Dynamic width calculation for custom selections (CPU + Clock = 450px)  |
| `monitor`   | `test_push_history_capacity`                                    | Capping the sparkline history Vec to a maximum of 30 items             |
| `monitor`   | `test_rotate_logs_succession`                                   | Log file shifting and generation renaming (`.1`, `.2`)                 |
| `monitor`   | `test_saturating_diff_calculation`                              | Network/IO delta calculation and underflow safety (`saturating_sub`)   |

---

## 3. Running Tests

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

## 4. Guidelines for Writing Tests

1. **In-Module Unit Testing**:
   - Implement unit tests within `#[cfg(test)] mod tests` in each module (`src/*.rs`) to keep tests close to their domain logic.
2. **Crash Prevention Checks**:
   - Test fallback paths to ensure that errors from sensor polling or OS-specific operations do not panic or crash the application.
