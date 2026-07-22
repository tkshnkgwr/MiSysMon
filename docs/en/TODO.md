**English** | [日本語版](../ja/TODO.md)

# Todo List: Mini System Monitor (MiSysMon)

This document tracks the development status, immediate tasks, and future backlog (feature expansion roadmap) of this project.

---

## 1. Implemented Features (Done)

- [x] **Basic UI Framework Construction**:
  - Slim horizontal bar (1100px x 32px) desktop window rendering using `egui` and `eframe`.
  - Borderless window, background transparency (Alpha: 200/255), always-on-top, and disabled resizability.
- [x] **Real-Time System Metrics Collection**:
  - **CPU**: Retrieve average CPU utilization across all cores using `sysinfo`.
  - **CPU Temp**: Detect CPU temperature from sensors labeled "CPU", "Core", "Package", etc. Implements a fallback to the maximum temperature of any sensor if no matching labels are found.
  - **Memory**: Retrieve used memory percentage.
  - **Network**: Aggregate rx/tx traffic across interfaces and calculate download/upload speeds based on the delta from the last update.
  - **Disk**: Display aggregate used space and total capacity of system disks in GB.
  - **Disk I/O**: Aggregate disk read/write bytes across all processes to display current I/O speeds (R/W).
- [x] **Diagnostics & Debugging**:
  - Write detected temperature sensors (labels and readings) to `sensors_debug.log` at startup.
- [x] **Utilities & Usability**:
  - Window dragging using the drag grip (`=`) on the left end.
  - Window position persistence (coordinates saved in JSON at shutdown and restored upon next startup).
  - Close button (`×`) on the right end.
  - Non-blinking fixed-width clock (`yyyy/mm/dd(Day) hh:mm:ss`) on the right end.
  - Disabled text selection to avoid layout shaking on accidental drags/clicks.
- [x] **Double-Launch Prevention**:
  - Mutex guard utilizing standard named mutexes via `common_lib`. If another instance exists, the new process terminates immediately.
- [x] **Initial Telemetry Spike Resolution**:
  - Recorded a baseline network traffic value during network module instantiation to resolve the bug where the transfer speed spikes enormously in the first second.
- [x] **Window Boundary Validation in Multi-Monitor Environments**:
  - Imported Win32 API `MonitorFromPoint` to validate coordinates and reset coordinates to the screen center if the saved coordinates fall outside active monitors.
- [x] **Diagnostic Log Rotation**:
  - Implemented log rotation up to 3 generations (`sensors_debug.log.1`, `.2`, `.3`) at startup.
- [x] **Window Auto-Snap**:
  - Snaps to desktop work area borders automatically when dragged within 15px (implemented via Win32 APIs).
- [x] **Resource Usage Charts (Mini Trendlines)**:
  - Added cyan sparklines (12px height line charts) representing the last 30 seconds of CPU and Memory telemetry data.
- [x] **Config Customization Panel**:
  - Double-clicking the grip (`=`) or right-clicking the main window displays a separate config dialog (sub-viewport).
  - Customizable items: update interval (0.5s–5.0s), window opacity (30%–100%), auto-snap toggle, and visibility checkboxes for CPU, Memory, Network, Disk, Disk IO, Version, and Clock.
  - Dynamic window width resizing to eliminate empty spaces when items are toggled.
- [x] **Absolute Screen Coordinate Dragging**:
  - Resolved dragging jitter/cursor drift by locking coordinates via Win32 API `GetCursorPos` delta tracking.
- [x] **Config Panel Display Placement in Multi-Monitor Environments**:
  - Fixed config panel coordinates to open on the same monitor as the main window by setting `.with_position()` relative to the main window.
- [x] **Dynamic Width Calculations & Layout Fixes**:
  - Refined required width metrics for each component in `calculate_width()` to prevent overlapping elements.
  - Optimized calls to `ViewportCommand::InnerSize` to fire only on width changes.

---

## 2. Immediate Tasks (In Progress / Todo)

- No outstanding tasks.

---

## 3. Future Backlog (Backlog)

- (No pending future features. Feature requests will be listed here.)
