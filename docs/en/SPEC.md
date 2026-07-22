**English** | [日本語版](../ja/SPEC.md)

# Specification: Mini System Monitor

**Version**: 1.0.0
**Internal Version**: 1.0.0.0

## 1. Overview
An extremely lightweight system monitor designed to run on Windows in low-resource environments.
It stays always-on-top and monitors system resources in real-time (at 1-second intervals by default).

## 2. Technical Stack
- **Prototype:** React, Tailwind CSS, Framer Motion
- **Production:** Rust, egui, eframe (0.35+), sysinfo (0.38+), chrono, serde, serde_json, common_lib (local shared crate)

## 3. Visual Specifications
- **Size:** 1100px x 32px (horizontal slim bar)
- **Decoration:** No title bar (Borderless)
- **Background:** Semi-transparent (Alpha: 200/255)
- **Font:** Highly visible sans-serif
- **Placement:** Always on Top

## 4. Functional Specifications
- **CPU Monitoring:** Displays the average utilization across all cores using `sysinfo`.
- **CPU Temperature:** Displays temperature retrieved from CPU hardware sensors.
- **Memory Monitoring:** Displays the percentage of used memory / total memory.
- **Network Monitoring:** Displays total upload/download speeds (Upload `^` / Download `v`).
- **Disk Space Monitoring:** Displays used space and total capacity of the system disk (e.g. 123GB/512GB).
- **Disk I/O Monitoring:** Displays real-time aggregate read (R) / write (W) speeds calculated from all running processes.
- **Version Display:** Displays the application version (in `vX.Y.Z` format) between the IO display and clock.
- **Clock Display:** Displays the current local time in `yyyy/mm/dd(Day) hh:mm:ss` format at the rightmost end.
- **Window Dragging:** Move the window by dragging the grip icon `=` on the left end. Implements an absolute screen mouse coordinate anchor mechanism to prevent lagging or cursor drift.
- **Closing App:** Terminate the application using the "×" close button at the right end.
- **Coordinate Persistence:** Automatically saves the window position at shutdown and restores it on the next startup. If the saved position is out of bounds (e.g., monitor configuration changes), it fallback-resets to the screen center.
- **Double-Launch Prevention**: Implemented via standard Win32 Named Mutex wrapper in `common_lib`. If another instance is running, the new process exits immediately.
- **Log Rotation:** Diagnostic logs `sensors_debug.log` are rotated up to 3 generations (`.1`, `.2`, `.3`) at startup to restrict disk consumption. Adds advice to run with admin privileges if 0 sensors are detected.
- **Auto-Snap**: Snaps to desktop work area edges (top, bottom, left, right) automatically when dragged within 15px. Can be toggled on/off in settings.
- **Mini Trendline**: Displays a 30-second history of CPU and Memory usage as 12px height sparklines (line charts).
- **Configuration Panel**: Double-clicking the handle (`=`) or right-clicking the main window opens a settings window on top. It automatically displays on the same monitor as the main window. Modifiable settings: update interval (0.5s–5.0s), background opacity (0.3–1.0), auto-snap toggle, and visibility checkboxes for CPU, Memory, Network, Disk, IO, Version, and Clock.
- **Dynamic Resize**: Automatically calculates the required width and resizes the main window dynamically when items are toggled on/off.
- **Usability:** Disables text selection to prevent misclicks.

## 5. Optimization
- **Refresh Control:** Telemetry data is refreshed on intervals synced with the configured update speed (0.5s–5s).
- **Layout Stabilization:** Assigns a fixed width to each component to prevent the layout from shaking due to changing digit counts.
- **API Adaptations:** Complies with eframe 0.35 and sysinfo 0.38 APIs.
- **Minimal Telemetry Scan:** Only refreshes required CPU and Memory metrics, skipping process resource telemetry except for process disk usage.
