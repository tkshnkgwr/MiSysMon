**English** | [日本語版](../ja/ARCHITECTURE.md)

# Architecture: Mini System Monitor (MiSysMon)

This document describes the architectural design, modular organization, technology stack, directory structure philosophy, and data flow of the lightweight Windows desktop monitor "Mini System Monitor (MiSysMon)".

---

## 1. System Overview and Goals

### Overview
Mini System Monitor (MiSysMon) is a slim, horizontal desktop overlay (1100px x 32px) designed for lightweight system monitoring.

### Goals
- **Minimal Footprint**: Keep CPU and RAM overhead near zero so that gaming or low-resource computing is never impacted.
- **High Visibility**: Display essential metrics (CPU usage/temp, RAM, network, disk, disk I/O, clock) in a clean, always-on-top, non-intrusive bar.
- **Usability**: Borderless semi-transparent window with smooth anchor-based dragging, edge auto-snapping, position persistence, and double-launch protection.

---

## 2. Technology Stack

The project utilizes the following crates and libraries:

| Layer / Library                    | Role                   | Rationale & Key Features                                                                             |
| :--------------------------------- | :--------------------- | :--------------------------------------------------------------------------------------------------- |
| **Rust** (Edition 2021)            | Programming Language   | High memory safety, zero-cost abstractions, low latency, and minimal resource footprint.             |
| **egui / eframe** (0.35.0)         | GUI Framework          | Immediate Mode lightweight GUI. Efficient GPU/CPU rendering suited for sleek overlays.               |
| **sysinfo** (0.38.0)               | System Metrics Engine  | OS-independent abstraction for CPU, memory, network, disk, and sensor telemetry.                     |
| **chrono** (0.4.38)                | Time & Date            | Local timestamp querying and format rendering.                                                       |
| **serde / serde_json**             | Serialization          | JSON persistence of window coordinates and user preferences.                                         |
| **common_lib** (Local Crate)       | Shared Utilities       | Local workspace crate providing Named Mutex double-launch guards and byte formatting (`format_bytes`).|

---

## 3. Architecture and Directory Structure

### Directory Layout

```
MiSysMon/
├── .agents/            # AI agent instructions and governance
│   ├── AGENTS.md       # Quality management and documentation rules
│   └── INSTRUCTIONS.md # AI coding guidelines
├── .github/            # GitHub Actions CI/CD workflows
├── docs/               # Architecture, specs, and design documentation
│   ├── ARCHITECTURE.md # This document
│   ├── DEVELOPMENT.md  # Local development guide
│   ├── DIAGRAM.md      # Mermaid diagrams
│   ├── FOOTPRINTS.md   # Footprint benchmarks
│   ├── RELEASE_FLOW.md # Release procedure guide
│   ├── SPEC.md         # Specification document
│   ├── TESTING.md      # Testing policies and procedures
│   ├── TEST_REPORT.md  # Test results report
│   └── TODO.md         # Task tracking and backlog
├── scripts/            # Build and utility scripts
├── src/                # Modular source code
│   ├── config.rs       # App configuration, defaults, serde persistence
│   ├── monitor.rs      # Telemetry polling, log rotation, history queue
│   ├── ui.rs           # Sparkline drawing, dynamic window width calculation
│   ├── win32.rs        # Win32 API integration, window auto-snapping
│   └── main.rs         # App entry point, eframe::App implementation
├── Cargo.toml          # Rust package manifest
└── CHANGELOG.md        # Change log
```

### Module Responsibilities
- **`src/config.rs`**: Application configuration data model (`Config`), JSON serialization/deserialization, and safe defaults fallback.
- **`src/win32.rs`**: Win32 API interop (absolute cursor coordinates, monitor detection, work area bounds) and 15px border snapping calculations (`apply_snap`).
- **`src/ui.rs`**: Sparkline trend rendering (`draw_trend_line`) and dynamic width calculation (`calculate_window_width`).
- **`src/monitor.rs`**: System telemetry polling via `sysinfo`, diagnostic log rotation (`rotate_logs`), and history queue management.
- **`src/main.rs`**: Main application entry point, `eframe::App` event loop, and double-launch orchestration via `common_lib`.

---

## 4. Data Flow and Inter-Module Workflow

### A. Startup and Initialization Flow
1. **Double-Launch Verification**:
   Calls `common_lib::desktop::acquire_single_instance` to verify the named mutex. If already running, the new instance exits immediately.
2. **Configuration Loading & Coordinate Validation**:
   Loads saved settings (`Config`) from persistent storage. On Windows, validates coordinates against active monitors via Win32 API `MonitorFromPoint`.
3. **Dynamic Size Allocation & Window Creation**:
   Dynamically calculates initial window width via `calculate_window_width()` and requests `ViewportCommand::InnerSize`.

### B. Event Loop and Telemetry Polling Flow

```
 ┌────────────────────────────────────────────────────────┐
 │                      eframe/egui                       │
 │                   (Immediate Mode)                     │
 └──────────────────────────┬─────────────────────────────┘
                            │ (Triggered on configured interval)
                            ▼
                  [ update_stats() ]
                            │
         ┌──────────────────┼──────────────────┐
         ▼                  ▼                  ▼
   sysinfo::System   sysinfo::Networks   sysinfo::Disks
   (CPU, Memory)        (Traffic)         (Disk Space)
         │                  │                  │
         └──────────────────┼──────────────────┘
                            ▼
              [ Delta Calculation & Storage ]
               - CPU Usage / Temp
               - Memory Usage
               - Network Speed (Up/Down)
               - Disk I/O Rate / Space
                            │
                            ▼
              [ History Buffer Storage ]
               - 30-second rolling telemetry buffer
                            │
                            ▼
                   [ ui() Render Loop ]
        - Render enabled metric components
        - Render mini sparkline trendlines
        - Custom drag & Win32 WorkArea auto-snap
        - Clock display (chrono) / Close button (×)
        - Right-click / Double-click ──> Config Viewport
```

### C. Configuration Customization (Sub-Viewport)
- Real-time reactivity: UI settings apply immediately to the main overlay.
- Dynamic resizing: Toggling metric items re-evaluates `calculate_window_width()` and updates the window width dynamically.

### D. Diagnostic Telemetry & Log Rotation
- During startup (`SystemMonitor::new`), detected sensors are logged to `sensors_debug.log`.
- Automatically rotates up to 3 backup generations (`.1`, `.2`, `.3`) on each launch.
