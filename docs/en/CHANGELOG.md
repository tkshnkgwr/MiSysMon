**English** | [日本語版](../ja/CHANGELOG.md)

# Changelog

## [2026-07-21]
### Fixed
- **Revamped Window Dragging Based on Absolute Screen Mouse Coordinates**: Introduced an anchor-point mechanism using Win32 `GetCursorPos` to fix the issue where the window flies away from the cursor or shakes/duplicates during drag movement. Achieved perfect synchronization with the mouse pointer and real-time snapping.
- **Config Window Display Position in Multi-Display Environments**: Explicitly set `.with_position()` relative to the main window coordinates for the config window (`ViewportBuilder`), fixing the issue where the settings screen would open on a different monitor.
- **Enhanced Screen-Edge Auto-Snap Snapping Detection**: Changed the reference point of the Win32 API call to fetch the monitor when snapping from the top-left of the window to the center coordinates `(center_x, center_y)`, preventing snap failures at the right/bottom edges of the screen and multi-monitor boundaries.
- **Resolved Element Overlaps and Insufficient Widths (IO/VER/Date-Time)**: Refined the calculation of the required width for each component (CPU, MEM, NET, DISK, IO, VER, CLOCK) in `calculate_width()` to match actual rendering sizes, resolving visual overlap bugs.

### Optimized
- **Optimized Window Resize Commands**: Optimized the call to `ViewportCommand::InnerSize` (which was previously called unconditionally every frame) to be invoked only when the width changes from the previous frame, reducing rendering load and OS-level processing delays.

## [1.0.0] - 2026-07-17
### Added
- **Window Auto-Snap Function**: Implemented a snapping feature using Win32 APIs that automatically snaps the window to the edges of the desktop working area (excluding the taskbar) when dragged within 15px.
- **Resource Usage Graph (Mini Trendline)**: Added real-time rendering of the last 30 seconds of history as a 12px height cyan line chart next to the CPU/Memory usage.
- **Settings Customization Screen (Separate Viewport)**: Double-clicking the grip `=` or right-clicking the main window displays a separate config popup. Modifiable items include update interval (0.5s–5.0s), opacity, snap toggle, and visibility of each item.
- **Dynamic Window Resize Function**: Implemented dynamic resizing of the main window according to the width of currently visible components when items are toggled on/off.
- **Markdown Exception for AI Pre-verification Process**: Added a rule to `.agents/AGENTS.md` allowing the AI Agent (Daikenja) to skip local verification processes (clippy, fmt, test, doc) when modifying only Markdown (`*.md`) files.

### Fixed
- **Fixed GitHub Actions Workflows (ci.yml / release.yml)**: Adjusted workflow actions to stable versions (`checkout@v4`, `action-gh-release@v2`) to fix execution errors caused by undefined `actions/checkout@v7` and `softprops/action-gh-release@v3`.
- **Fixed Cross-platform Compilation (Clippy) Errors**: Added `#[allow(unused_mut)]` to suppress `unused_mut` warnings (treated as errors) on non-Windows targets (Linux/macOS) for the snapping variable `new_pos`.

### Removed
- **Abolished Auto Version Bump Workflow**: Deleted the automatic version bump workflow (`bump-version.yml`) to prevent unnecessary automated commits and merge conflicts on GitHub. Version bumps and tags are now managed manually by developers.

## [2026-07-16]
### Added
- **Multi-Monitor Window Position Validation**: Implemented a fallback logic using Win32 API `MonitorFromPoint` (via raw link to suppress binary size increases) to reset coordinates to the screen center if the saved position is out of bounds (due to monitor disconnection or resolution changes).
- **Log Rotation Feature**: Added a feature to rotate CPU temperature diagnostic logs (`sensors_debug.log`) up to 3 generations at startup.
- **Documentation**:
  - Created architecture document [ARCHITECTURE.md](docs/ARCHITECTURE.md).
  - Created AI instruction document [.agents/INSTRUCTIONS.md](.agents/INSTRUCTIONS.md).
  - Created todo list [TODO.md](docs/TODO.md).

### Fixed
- **Fixed Network Bandwidth Display Spike at Startup**: Prevented the cumulative transfer bytes from spiking at startup by recording the baseline network bytes beforehand.
- **Added Advice to CPU Temp Diagnostic Log**: Appended an advice to the log suggesting running the app with administrator privileges when 0 sensors are detected.
- **Standardized Document Naming Conventions**: Standardized all documentation filenames to uppercase snake case (e.g., `README.ja.md` ➡️ `README_JA.md`, `project_template_guide.md` ➡️ `PROJECT_TEMPLATE_GUIDE.md`).

## [2026-07-14]
### Added
- **Added Rustdoc Comments**: Added detailed documentation comments (`///` and `//!`) for all structs (`Config`, `SystemMonitor`), fields, methods (`new`, `update_stats`, `save`, `ui`), and `main` function in `src/main.rs`.
- **Updated AI Instruction (AGENTS.md)**: Updated rules to ensure Rustdoc comments are synchronized on code changes, and added private items check (`cargo doc --no-deps --document-private-items`) to the local pre-verification process.

## [2026-07-13]
### Added
- **Updated AI Instruction (AGENTS.md) / Markdown Change Exclusions**: Added a rule to skip automatic documentation (like changelogs) when modifying only Markdown (`*.md`) files. Also added notes that GitHub Actions CI will not trigger for Markdown-only changes.

### Optimized
- **Streamlined GitHub Actions Workflows (ci.yml / bump-version.yml)**: Added `paths-ignore: '**.md'` to avoid redundant builds/tests and version bumps for commits that only modify Markdown files.

## [2026-07-10]
### Added
- **Added `docs/RELEASE_FLOW.md` (Release Manual)**: Documented version bumping via GitHub Actions, draft release creation with tag pushes, PAT registration, manual build trigger instructions, and troubleshooting.
- **Added `docs/DEVELOPMENT.md` (Multi-Repository Development Guide)**: Documented the local directory layout and GitHub Actions multiple-checkout design for projects dependent on `common_lib` via relative path.
- **Added Links to Internal Docs in README**: Linked the newly added documents in `README.md` and `README_JA.md`.

### Optimized
- **Migrated Shared Utilities**: Migrated the byte formatting logic `format_bytes` to `src/text.rs` of the shared library `common_lib`, and removed duplicate code and tests from `main.rs`.

## [2026-07-06]
### Fixed
- **Normalized GitHub Actions Workflows**:
  - Restored `actions/checkout` and `softprops/action-gh-release` to stable versions (`v4` and `v2`, respectively).
  - Configured workflows to clone `common_lib` alongside `MiSysMon` to resolve the relative dependency (`../common_lib`). Implemented a fallback to `github.token` if `secrets.PAT` is missing.
  - Set default working directory for all jobs to `MiSysMon` and corrected asset paths.
- **Updated AI Instruction (AGENTS.md)**: Added rules to maintain shared library dependencies (`common_lib`) when editing workflow definitions in the future.

## [2026-07-03]
### Fixed
- **Fixed Cargo.toml Path Bug**: Fixed the dependency path of `common_lib` from `../../common_lib` to the correct `../common_lib`.

### Optimized
- **Cleaned Up Code via Shared Library Integration**: Removed all raw unsafe Windows API (FFI) code from `main.rs` and migrated to `desktop::acquire_single_instance` provided by `common_lib`.

## [2026-06-30]
### Added
- **Added Latest Release Badge**: Added GitHub Latest Release badges to `README.md` and `README_JA.md`.
- **Updated AI Instruction (AGENTS.md)**: Added rules to maintain and update release badges and multi-language links in READMEs.

### Optimized
- **Verified Rust Environment Upgrade**: Conducted tests and checks after upgrading local Rust to `1.96.0`. Executed `cargo test`, `cargo clippy`, and `cargo fmt` to verify that there are no warnings or errors.

## [2026-06-26]
### Added
- **Added CI (GitHub Actions) Configuration**: Configured automated builds, tests, formatting checks (`cargo fmt`), and lint checks (`cargo clippy`) on `ubuntu-latest`, `macos-latest`, and `windows-latest`.
- **Added CI Status Badge**: Added GitHub Actions CI badges to `README.md` and `README_JA.md`.
- **Added Unit Tests**: Added unit tests in `src/main.rs` for formatting bytes (`format_bytes`) and default configuration values (`Config::default`).
- **Introduced Dependabot**: Set up Dependabot for automated Cargo dependencies and GitHub Actions updates.
- **Added Automated Release Workflow**: Set up a workflow to automatically build optimized Windows executables (`mini-system-monitor.exe`) and upload them as GitHub Release drafts when pushing version tags (`v*`).
- **Added EditorConfig**: Added a `.editorconfig` file to unify code style configurations across editors.
- **Added Auto Version Bump Workflow**: Added a workflow to automatically increment the patch version and push tags on merge/push to the `main` branch. This triggers the release draft creation automatically.
- **Updated Template Guide & AI Instruction**: Added config templates and descriptions in `docs/project_template_guide.md`. Added local pre-verification rules (test, clippy, format) and workflow file protection rules to `.agents/AGENTS.md`.
- **Added Version Display**: Added `VER vX.Y.Z` between the IO display and clock in the UI.

### Fixed
- **Fixed Clippy Warnings**: Replaced manual `Default` implementations in `src/main.rs` with `#[derive(Default)]` to resolve `clippy::derivable_impls` warnings.

### Optimized
- **Formatted Code**: Executed `cargo fmt` to clean up code formatting.
- **Updated Dependencies and Kept API Compatibility**: Upgraded `eframe` to `0.35.0` and `sysinfo` to `0.38.0` (highest version compatible with Rust 1.94.1 in development env). Optimized `src/main.rs` for the new APIs.

## [2026-06-25]
### Added
- **Added Auto-Documentation Rules**: Created AI Agent instructions `.agents/AGENTS.md` to define processes for checking and updating `CHANGELOG.md` and `SPEC.md` on code changes.
- **Added Automation Scripts**: Added `scripts/` directory containing `bump-version.ps1` to sync versions and generate/update documents. Added metadata headers to `SPEC.md` and `TEST_REPORT.md`.

### Fixed
- **Fixed Weekday Notation in README_JA.md**: Fixed example weekday representation from `(火)` to `(Tue)` in `README_JA.md` to align with the actual UI rendering (always in English abbreviations).

### Optimized
- **Organized Internal Documentation**: Moved internal design and report documents (`SPEC.md`, `DIAGRAM.md`, `FOOTPRINTS.md`, `TEST_REPORT.md`) from the root directory to `docs/` to keep the root tidy.
- **Revised Template Guide**: Rewrote `docs/project_template_guide.md` from a Tauri v2 configuration to a pure Rust desktop application template using eframe/egui.

## [2026-06-24]
### Fixed
- **Fixed Disk I/O Rate Bug**: Fixed a bug where duplicate subtraction was performed for disk reads/writes. `sysinfo` already returns the delta of bytes since the last refresh, so manual calculation was incorrect.
- **Reduced CPU Usage via sysinfo Refresh Optimization**:
  - Removed global `refresh_all()` and switched to targeted updates (`refresh_cpu()` and `refresh_memory()`).
  - Restricted process updates to disk usage only (`ProcessRefreshKind::new().with_disk_usage()`) to skip unnecessary telemetry data collection.
  - Initialized `sysinfo` using `System::new()` instead of complete scans to reduce startup overhead.
- **Moved CPU Temp Diagnostic Log for Safety**: Changed the path of `sensors_debug.log` to `%APPDATA%\Mini System Monitor` to prevent writing errors in startup folder and keep the current folder clean.

### Added
- **Added Single Instance Guard**: Implemented double-launch prevention using Windows named mutexes. The app terminates immediately if another instance is running.
- **Added MIT License File**: Added formal `LICENSE` file in the root.
- **Enhanced Documentation (README.md / README_JA.md)**:
  - Added guide for auto-startup (常駐化) setup on Windows.
  - Added physical file paths for persistent configuration and reset procedures.

## [2026-05-07]
### Fixed
- **Robust CPU Temp Retrieval**: Added fallback components search (Tctl/Tdie and Thermal labels) for AMD environments. Displays `--` instead of `0.0` when no data is fetched.
- **Fallback for CPU Temp sensor**: Implemented a fallback to take the maximum temp of any component when no explicit CPU labels are found.

## [2026-04-16] - Final Release Candidate
### Added
- **Disk I/O Display**: Displays the aggregate read/write rate of all processes.
- **Network Speed Display**: Improved display to show transfer rate per second rather than cumulative transfer bytes.
- **Window Position Persistence**: Saved window coordinates at exit to restore them upon next startup.
- **CPU Temp Display**: Added CPU temp collection using `sysinfo::Components`.
- **Release Optimization**: Specialized `Cargo.toml` profiles for size optimization and symbol stripping.

### Fixed
- **Hide Console Window**: Appended `#![windows_subsystem = "windows"]` to avoid displaying the console window on Windows.
- **Rigid Window Size**: Enforced window dimensions at startup to avoid size growth.
- **Resolved Dependencies**: Locked `serde` and other crate versions to fix compile errors.
- **Fix Character Display**: Swapped special characters (↑↓☷) to standard characters (^v=) for universal font support.
- **API Adaptations**: Adapted to eframe 0.27 and sysinfo 0.30.

### Removed
- **Visiblity Toggles**: Removed visibility toggle controls due to instability.

## [2026-04-15] - Initial Prototype & Rust Design
### Added
- Created initial prototype using React/Tailwind.
- Implemented Rust port blueprint (`Cargo.toml`, `main.rs`).
- Framed specifications for transparency, borderless style, and always-on-top behavior.
