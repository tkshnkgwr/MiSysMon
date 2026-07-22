**English** | [日本語版](../ja/ARCHITECTURE.md)

# Architecture: Mini System Monitor (MiSysMon)

This document describes the design structure, technical stack, directory structure intent, and data flow of the lightweight system monitor application "Mini System Monitor (MiSysMon)" for Windows environments.

---

## 1. System Overview and Purpose

### Overview
Mini System Monitor (MiSysMon) is a lightweight, slim horizontal bar (1100px x 32px) system monitor that stays on top of the screen.

### Purpose
- **Minimal Footprint**: Keep memory usage and CPU utilization extremely low so as not to interfere with low-spec PCs or gaming environments.
- **High Visibility**: Stay always on top, providing a compact and at-a-glance view of CPU usage/temperature, memory usage, network traffic, disk usage/IO, and current date/time.
- **Accidental Operation Prevention & Convenience**: While being borderless and transparent, it features smooth dragging without lag, window position restoration on the next startup, and double-launch prevention.

---

## 2. Technical Stack

The project consists of the following technologies and libraries:

| Layer / Library               | Role                      | Reasons / Features                                                                                    |
| :---------------------------- | :------------------------ | :---------------------------------------------------------------------------------------------------- |
| **Rust** (Edition 2021)       | Development Language      | High memory safety, zero-cost abstractions, low latency, and minimal resource footprint.              |
| **egui / eframe** (0.35.0)    | GUI Framework             | Immediate Mode lightweight GUI. Efficient GPU/CPU rendering, suitable for slim UI implementations.   |
| **sysinfo** (0.38.0)          | System Metrics Collection | Collects CPU, memory, network, disk, and temperature information via an OS-independent interface.   |
| **chrono** (0.4.38)           | Date & Time               | Local date/time acquisition and formatting.                                                           |
| **serde / serde_json**        | Serialization             | Serializes config information (like window position) for persistent storage.                          |
| **common_lib** (Local Crate) | Shared Library            | Shared library located in the parent directory. Used for acquiring the single instance mutex and formatting bytes (`format_bytes`). |

---

## 3. Architecture and Directory Structure Intent

### Directory Structure

```
MiSysMon/
├── .agents/            # AI Agent Instructions and Rules
│   └── AGENTS.md       # Auto-documentation and quality control rules
├── .github/            # GitHub Actions workflow configurations
├── docs/               # Project specifications, designs, and other docs
│   ├── en/             # English documentation
│   │   ├── ARCHITECTURE.md # This file
│   │   └── ...
│   └── ja/             # Japanese documentation
│       ├── ARCHITECTURE.md
│       └── ...
├── scripts/            # Build and distribution scripts
├── src/                # Source code
│   └── main.rs         # All application logic (Entrypoint, UI, metrics updates)
├── Cargo.toml          # Rust package configuration
└── TODO.md             # Removed or moved? (Now located in docs/en/TODO.md and docs/ja/TODO.md)
```

### Structural Intent
- **Extreme Simplification via Single Entry Point**:
  Due to the small scale of the application, all logic (UI, system metrics collection, settings persistence, and main loop) is housed in a single file `src/main.rs`. This minimizes code overhead and allows the compiler to maximize optimization (LTO).
- **Separation of Shared Logic (`common_lib`)**:
  Low-level processes such as platform-specific APIs and formatting are separated into the adjacent shared library `common_lib`, preventing the core MiSysMon code from bloating.
- **Strict Release Optimization Settings (`Cargo.toml`)**:
  Profile settings are specialized for binary size reduction, using `opt-level = 'z'` (size optimization), `strip = true` (symbol stripping), and `panic = 'abort'` (unwind removal).

---

## 4. Data Flow and Inter-module Cooperation

### A. Startup and Initialization Flow
1. **Single Instance Check**:
   Checks if a specific named mutex exists on the system via `common_lib::desktop::acquire_single_instance`. If already running, the process terminates immediately and normally.
2. **Config Load & Position Validation**:
   Restores configuration (`Config`) serialized during the last exit from `eframe`'s persistent storage.
   On Windows, the restored coordinates are validated against active monitor areas using Win32 API `MonitorFromPoint`. If out of bounds, the window is reset to the initial screen center.
3. **Dynamic Initial Size Calculation & Window Creation**:
   Dynamically calculates the initial window width based on active items (CPU, Memory, etc.) via `calculate_width()`, locks the size via `ViewportCommand::InnerSize`, and creates/displays the window.

### B. Event Loop and Rendering/Update Data Flow

```
  ┌────────────────────────────────────────────────────────┐
  │                      eframe/egui                       │
  │                   (Immediate Mode)                     │
  └──────────────────────────┬─────────────────────────────┘
                             │ (Called at configured update intervals)
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
              [ Difference Calculation & Storage ]
                - CPU usage / Temperature
                - Memory usage
                - Network speed (Up / Down)
                - Disk IO speed / Usage
                             │
                             ▼
              [ Buffer Storage for History Data ]
                - Keeps last 30 seconds of CPU & Mem data
                             │
                             ▼
                    [ ui() Rendering Loop ]
         - Layouts only active items (toggled)
         - Renders mini trendline from history data
         - Custom dragging & auto-snap (Win32 WorkArea snapping)
         - Clock display (chrono) / Close button (×)
         - Right-click/Double-click detection ──> Config Dialog
```

### C. Config Customization (Separate Viewport)
- Configuration changes are reflected in the main UI in real-time.
- When display items are toggled, `calculate_width()` is re-evaluated, and the main window size changes dynamically.

### D. Diagnostic Log & Log Rotation
- During initialization (`SystemMonitor::new`), detected temperature sensor information is output to `sensors_debug.log` for diagnostics.
- Each time the app starts, past logs are rotated up to 3 generations (`sensors_debug.log.1`, `.2`, `.3`) to keep disk space usage constant.
- If 0 sensors are detected, an advice to run the app with Administrator privileges is automatically appended to the log.

1. **Data Acquisition Optimization**:
   To avoid wasted CPU cycles every frame, `sysinfo` refresh is deferred until the configured update interval (0.5s–5s) has elapsed, keeping window redraws to a minimum.
2. **Custom Dragging & Auto-Snap**:
   Tracks movement delta from the drag handle `=` and automatically clamps (snaps) the window to the border when it gets within 15px of a Windows WorkArea (`RECT`) boundary.
3. **Safe Shutdown and Persistence**:
   When the user clicks the "×" close button, a close command is sent. When `eframe` terminates, the `save` method is called to serialize and save the final coordinates and custom config values (opacity, update interval, toggles, etc.) to disk.
