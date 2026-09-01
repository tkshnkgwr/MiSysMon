**English** | [日本語版](../ja/TEST_REPORT.md)

# Test Report: Mini System Monitor (MiSysMon)

**Target Version**: mini-system-monitor v1.1.0

## 1. Test Environment
- **OS:** Windows 11 / Windows 10
- **Rust Toolchain:** 1.80+ (stable-x86_64-pc-windows-msvc)
- **Key Crates:** eframe 0.35.0, sysinfo 0.38.0, chrono 0.4.38, serde 1.0.203, common_lib 0.2.4

---

## 2. Unit Test and Verification Results

Executed all 15 automated unit tests and functional checks, confirming that all items passed successfully:

| Test Category           | Test Item / Function                                           | Result | Notes / Methodology                  |
| :---------------------- | :------------------------------------------------------------- | :----: | :----------------------------------- |
| **Config Defaults**     | `Config` default value initialization consistency              |  Pass  | `test_config_default_values`         |
| **Config Roundtrip**    | JSON serialization & deserialization fidelity                  |  Pass  | `test_config_json_serialization...`  |
| **Config Fallback**     | Safe default fallbacks on missing or partial JSON keys         |  Pass  | `test_config_json_partial_fallback`  |
| **Snap (Left Border)**  | Auto-snap within 15px of left border and negative coords       |  Pass  | `test_apply_snap_left_edge`          |
| **Snap (Right Border)** | Auto-snap within 15px of right border                          |  Pass  | `test_apply_snap_right_edge`         |
| **Snap (Top Border)**   | Auto-snap within 15px of top border                            |  Pass  | `test_apply_snap_top_edge`           |
| **Snap (Bottom Border)**| Auto-snap within 15px of bottom border                         |  Pass  | `test_apply_snap_bottom_edge`        |
| **Snap (No Snap)**      | No snap outside 15px threshold (position preserved)            |  Pass  | `test_apply_snap_no_snap_when...`    |
| **Snap (Corners)**      | Simultaneous 2-axis snapping at screen corner boundaries       |  Pass  | `test_apply_snap_corner_snap`        |
| **UI Width (All On)**   | Required width calculation with all elements enabled (1210px)  |  Pass  | `test_calculate_window_width_all...` |
| **UI Width (Min Clamp)**| Minimum width clamp with all elements disabled (200px)         |  Pass  | `test_calculate_window_width_all...` |
| **UI Width (Custom)**   | Accurate dynamic width calculation for custom selections       |  Pass  | `test_calculate_window_width_cus...` |
| **History Queue**       | Sparkline Vec capacity capping (30 items) with FIFO sliding    |  Pass  | `test_push_history_capacity`         |
| **Log Rotation**        | `sensors_debug.log` multi-generation shifting (`.1`, `.2`)     |  Pass  | `test_rotate_logs_succession`        |
| **Telemetry Delta**     | Cumulative delta calculation and underflow clamp (`saturating`)|  Pass  | `test_saturating_diff_calculation`   |
| **Static Analysis**     | `cargo clippy --all-targets -- -D warnings` zero warnings      |  Pass  | Clippy warnings fully resolved       |
| **Formatting**          | `cargo fmt --check` full formatting compliance                 |  Pass  | Formatted via rustfmt                |
| **Documentation**       | `cargo doc --no-deps --document-private-items` zero warnings   |  Pass  | Comprehensive RustDoc comments       |
| **Double Launch Lock**  | Prevents duplicate running instances                           |  Pass  | `common_lib` Named Mutex             |
| **Anchor Dragging**     | Smooth window dragging synced to cursor via `GetCursorPos`     |  Pass  | Zero jitter or displacement          |

---

## 3. Performance Metrics
- **CPU Overhead:** < 0.1% (minimized refresh workload and selective process telemetry polling).
- **Memory Footprint:** ~15–20 MB steady-state resident memory.
- **Binary Footprint:** Ultra-lightweight release profile via `opt-level = 'z'`, `lto = true`, `panic = 'abort'`, and `strip = true`.
