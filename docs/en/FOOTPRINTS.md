**English** | [日本語版](../ja/FOOTPRINTS.md)

# Footprints: Mini System Monitor Development Journey

This is a comprehensive record of the technical transitions, backgrounds of decision-making, and modifications in the development of this project.

## 🐾 MileStone 1: Prototype Phase (Decision to move React -> Rust)
- **Challenge:** In the React/Vite prototype, the overhead of the browser engine was non-negligible in low-resource Windows environments.
- **Decision:** To achieve extreme low-load, decided to completely port to Pure Rust (egui/eframe) without Webview.
- **Outcome:** Reduced memory consumption from tens of megabytes to a few megabytes, and CPU load to almost 0%.

## 🐾 MileStone 2: Rust Stabilization & API Adaptation
- **Technical Barrier:** The major update of the `sysinfo` crate to version 0.30 significantly changed the data retrieval API.
- **Modifications:** Abolished legacy traits such as `SystemExt` and rewrote it using the new `refresh_all()` and component retrieval flow.
- **Outcome:** Built a codebase that compiles stably with the latest dependencies.

## 🐾 MileStone 3: Refined UI/UX and Measures Against Character Encoding Issues (Tofu)
- **Challenge:** Due to lack of system fonts in low-resource environments, special characters (↑, ↓, ☷) were rendered as blank boxes ("Tofu").
- **Modifications:** Replaced symbols with standard text chars (^, v, =).
- **Improvement:** Applied fixed width using `add_sized` to each item, resolving UI jitter caused by varying digits.

## 🐾 MileStone 4: Persistence and Window Behavior Control
- **Feature Addition:** Introduced `serde` and implemented window coordinate persistence in JSON.
- **Difficulty:** A strange phenomenon occurred where the height of the bar automatically expanded to 2-3x upon restart.
- **Solution:** Locked `min_inner_size` and `max_inner_size` to the exact same value in `NativeOptions`, and successfully contained the behavior by forcing the `InnerSize` command during the startup sequence.

## 🐾 MileStone 5: Full Windows Support and Release Optimizations
- **Challenge:** Unnecessary console windows ("DOS windows") appeared behind the GUI app during debug builds or standard execution.
- **Modifications:** Added the `#![windows_subsystem = "windows"]` attribute to explicitly define it as a GUI-exclusive application.
- **Optimization:** Added `strip = true` and `lto = true` to `Cargo.toml` to minimize the binary size.

## 🐾 MileStone 6: Advanced Telemetry Items (Disk I/O)
- **Challenge:** `sysinfo` does not directly provide real-time aggregate I/O speeds for physical disks.
- **Solution:** Created a custom logic to aggregate `disk_usage` across all processes and calculate the delta from the previous second. Network speeds were similarly converted from cumulative traffic to rate-per-second.

## 🐾 MileStone 7: CPU Temperature Retrieval (2026-05-07)
- **Challenge:** In environments with specialized sensor labels (Tdie, Tctl, etc.) or undefined labels, the CPU temperature was constantly displayed as 0.0°C.
- **Improvement 1:** In addition to standard label matching (CPU/CORE/PACKAGE), added searches for AMD-specific `TCTL`, `TDIE`, and general `THM` (Thermal) sensors.
- **Improvement 2:** If none are found, the value of the highest temperature sensor in the system is adopted as the CPU temperature.
- **UI Tweaks:** Changed display from `0.0°C` to `--°C` when temperature retrieval fails to avoid confusion.
- **Diagnostics:** Configured writing a `sensors_debug.log` to the execution directory at startup.
- **Conclusion:** Identified that if `Detected Sensors Count: 0` occurs, the telemetry library (`sysinfo`) cannot fetch sensor data through the WMI standard APIs.
- **Design Decision:** Shipping kernel ring-0 drivers is contrary to the principle of "low resource, secure, and lightweight". Therefore, displaying `--°C` when retrieval fails was finalized.

## 🐾 MileStone 8: Disk I/O Bug Fix, Telemetry Optimization, and Documentation (2026-06-24)
- **Challenges:**
  1. Disk I/O rate was only updated immediately after startup and stuck to 0 from the second second onwards.
  2. Running `refresh_all()` every second scanned the entire system, causing unnecessary CPU overhead.
  3. LICENSE file was missing, and guides for auto-startup and settings reset were lacking.
  4. Diagnostics log `sensors_debug.log` was created directly in the current directory, resulting in files created in unexpected folders (e.g. at startup) or writing failures due to lack of write permissions.
  5. The application lacked double-launch prevention, risking multiple instances overlapping on screen and wasting resources.
- **Cause:** `Process::disk_usage()` from `sysinfo` already returns the delta of bytes since the last update. The program was incorrectly performing a secondary delta subtraction. Also, log destination depended on Cwd, and double-launch protection was missing.
- **Modifications:**
  - Removed custom delta subtraction logic, showing the rate values directly.
  - Abolished `sys.refresh_all()`, targeting updates to CPU/Memory and limiting process scans to disk usage (`ProcessRefreshKind::new().with_disk_usage()`).
  - Added MIT `LICENSE` and documented startup procedures, actual configuration file paths (`%APPDATA%`), and reset steps in READMEs.
  - Moved the diagnostic log path to `%APPDATA%\Mini System Monitor` (same as settings file) and updated documents.
  - Implemented double-launch guard using standard library and Win32 named mutex APIs (`CreateMutexW`).
- **Outcome:** Correct disk I/O rates are now displayed, and unnecessary system scans are eliminated, dropping CPU overhead significantly. Moving the diagnostics log keeps the working directory clean and ensures logs are generated reliably. Double-launch guard prevents UI overlap and memory bloat.

## 🐾 MileStone 9: Shared Crate (common_lib) Integration and Code Simplification (2026-07-03)
- **Challenges:**
  1. The relative path of the shared crate `common_lib` in `Cargo.toml` was typoed as `../../common_lib`, causing build failures.
  2. Windows OS double-launch prevention was implemented via FFI in `main.rs`, leaving unsafe code blocks and redundant modules in the codebase.
- **Modifications:**
  - Fixed relative path of `common_lib` to `../common_lib` and enabled `windows_desktop` feature.
  - Fully deleted `windows_util` module (containing raw `CreateMutexW` / `GetLastError` FFIs) from `main.rs`.
  - Replaced double-launch prevention logic with `common_lib::desktop::acquire_single_instance`.
  - Assigned returned guard object to a local variable `_guard` in `main` for safety.
- **Outcome:** Redundant raw FFI declarations and unsafe blocks were fully removed, dramatically improving code maintainability. Proven module reusability through the shared crate.

---

## 📈 Technical Notes (FOOTPRINT Logs)

### 2026-04-16: Crate Resolution Iterations
- **Issue:** Build errors looped back and forth depending on the presence of `extern crate`.
- **Why:** While unnecessary in modern Rust (Edition 2021), parser quirks in local environments caused confusion.
- **Fix:** Standardized to standard `use` statements and explicitly set versions in `Cargo.toml`.

### 2026-04-16: Mystery of Inflating Window Size
- **Issue:** Window size went wild when restoring coordinates from `Pos2`.
- **Why:** Auto-scaling in eframe conflicted with manual position restoration.
- **Fix:** Appended manual size enforcement command at startup.

### 2026-04-21: GitHub README Reorganization
- **Intent:** Clearly highlighted our selling point: Native Rust is lighter than Tauri webviews.
