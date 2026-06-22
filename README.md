# MiSysMon (Mini System Monitor - Rust Edition)

📖 [日本語版はこちら (Japanese Version)](./README.ja.md)

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)

An ultra-lightweight, ultra-slim desktop system monitor optimized for low-resource environments (such as lower-spec Windows PCs).

| CPU | MEM | NET | DISK | IO | CLOCK |
| :---: | :---: | :---: | :---: | :---: | :---: |
| 5.1% (42°C) | 12.3% | 1.2M^ / 0.8Mv | 123G/512G | 0.5MR / 0.1MW | 2026/04/21 12:00:00 |

## 🌟 Features

- **Ultra-Lightweight Operation:** Built with pure Rust + `egui` (Immediate-mode GUI), resulting in significantly lower memory footprint and CPU utilization compared to WebView-based frameworks (like Tauri or Electron).
- **Slim & Space-Saving:** A horizontal bar design of 1100x32px. It fits neatly at the top or bottom of your desktop without interrupting your workflow.
- **Always on Top / Transparent Background:** Transparent background overlay that always stays on top of other windows so you can keep an eye on your system at all times.
- **Window Position Persistence:** Automatically remembers and restores the window position from the last session upon launching.
- **Zero Distraction / Native GUI:** Runs as a polished GUI application without opening a background Command Prompt (black DOS window).

## 📊 Monitored Metrics

- **CPU:** Usage (%) and Package Temperature (°C) (*Note: Displays `--°C` if sensor data is unavailable due to OS security limits or unmatched hardware. Check `sensors_debug.log` for diagnostics.*)
- **Memory:** Utilized percentage (%)
- **Network:** Active real-time uploads (^) and downloads (v) speeds
- **Disk Space:** Utilized space and total capacity of the system drive
- **Disk I/O:** Real-time disk read (R) and write (W) speeds
- **Clock:** Fully integrated date & time display with seconds formatting

## 🛠️ Setup and Build Instruction

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

### Build Steps
```powershell
# Clone the repository
git clone https://github.com/tkshnkgwr/MiSysMon.git
cd MiSysMon

# Build a release package (highly optimized single binary)
cargo build --release
```
The compiled binary executable will be generated at `target/release/mini-system-monitor.exe`.

## 🎨 Under the Hood Design Principles
To balance glanceable legibility and utility:
- **Impact-Style Typography:** Utilizes strong, clear, and highly visible fonts for critical numerical readings.
- **Zero Flicker / Fluid Refresh:** Excludes flashing, blinking, or unnecessary heavy animations to maintain a stable, non-intrusive layout.
- **Hardware Dial Aesthetic:** A sleek interface modeled after professional rack unit diagnostic panels.

## 🔍 Troubleshooting (CPU temperature not displaying)
Depending on your motherboard's vendor-specific sensor design or Windows access control/WMI permissions, CPU temperature polling might fail.
1. **Try elevation:** Right-click the executable and select **"Run as Administrator"**.
2. **Review Diagnostics:** Check `sensors_debug.log` generated automatically in the application directory on launch. If it says `Detected Sensors Count: 0`, the Windows standard API cannot interface with your motherboard's thermal sensors natively.

## 📄 License
MIT License
