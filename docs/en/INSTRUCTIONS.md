**English** | [日本語版](../ja/INSTRUCTIONS.md)

# AI Instructions: Coding Standards & Guidelines

This document defines the coding standards, conventions, and documentation synchronization rules that the AI Agent (Daikenja) must adhere to when proposing modifications, fixes, or additions to the source code of this project.

---

## 1. Naming Conventions
Strictly adhere to the standard Rust API Guidelines (RFC 430).
- **Types, Structs, Enums, and Traits**: `UpperCamelCase` (PascalCase)
- **Functions, Methods, Variables, Macros, and Modules**: `snake_case`
- **Constants & Static Variables**: `SCREAMING_SNAKE_CASE`
- **Filenames**:
  - Source code and scripts: `snake_case.rs`
  - Documentation files (Markdown, etc.): **SCREAMING_SNAKE_CASE (`SCREAMING_SNAKE_CASE.md`)**

---

## 2. Error Handling Policy
- **Avoid `panic!` and `unwrap()`**: Strictly avoid `.unwrap()` and `.expect()` in non-test production code.
- **Error Propagation via Return Types**: Return `Result<T, E>` or `Option<T>` for fallible operations and handle them gracefully at the call site.
- **Safe Defaults & Fallbacks**: Return safe default values (e.g., `0.0`) or apply fallback logic when hardware telemetry or file I/O fails.

---

## 3. Component & Module Division Criteria
- **Maintain Modular Architecture**: The project is organized into specialized modules (`config.rs`, `monitor.rs`, `ui.rs`, `win32.rs`, `main.rs`) by domain responsibility. Place new functionality in the appropriate module.
- **Enforce 1,000 LOC Limit**: If any single source file exceeds **1,000 lines**, proactively propose and execute further submodule refactoring.
- **Leverage Shared Library (`common_lib`)**: Import reusable cross-project utilities (such as single-instance mutex guards and byte formatting) from `common_lib`.

---

## 4. Automatic Documentation & Synchronization Rules

When modifying source code, refactoring, adding features, fixing bugs, or updating dependencies, inspect and update the following documentation suite as needed.
*Note: **When modifying only `.md` files, skip this automatic documentation cascade and the Rust pre-verification process.***

### 4.1 General Rules
- **Bilingual Synchronization**: Always update files in both `docs/en/` (English) and `docs/ja/` (Japanese) to keep them completely synchronized.
- **Language Switcher Links**: Place a reciprocal language switcher link at the very top of every document (e.g., `**English** | [日本語版](../ja/SPEC.md)`).

### 4.2 Target Documents & Update Triggers

| Document Name                    | Role                      | Update Trigger & Detailed Rules                                                                   |
| :------------------------------- | :------------------------ | :------------------------------------------------------------------------------------------------ |
| `CHANGELOG.md`                   | Change History            | Every implementation. Ordered newest first under `## [YYYY-MM-DD]` or version (`Added`/`Fixed`).  |
| `SPEC.md`                        | Functional & Visual Specs | Changes to window dimensions, monitored metrics, display formats, UI components, or target OS.    |
| `DIAGRAM.md`                     | System & Dataflow Diagram | Changes to telemetry sources, thread structures, event loops, or external API paths (Mermaid).    |
| `README.md` / `README_JA.md`     | Project Overview & Setup  | Changes to launch options, config paths, startup steps, or requirements. Keep badges & doc links. |
| `FOOTPRINTS.md`                  | Performance & Milestones  | Changes to release binary size, memory footprint, build optimization flags, or new milestones.    |
| `ARCHITECTURE.md`                | Architectural Design      | Changes to tech stack, module creation/splitting, directory layout, or data flow structures.      |
| `INSTRUCTIONS.md`                | AI Coding Guidelines      | Updates to naming conventions, error handling, modular criteria, or documentation sync rules.     |
| `TODO.md`                        | Task & Backlog Tracking   | Updates to Done (implemented), In Progress / Todo (immediate), or Backlog (future roadmap).       |
| `DEVELOPMENT.md`                 | Multi-Repo & CI Guide     | Changes to `common_lib` dependencies, recommended local folder layout, or GitHub Actions setup.   |
| `RELEASE_FLOW.md` / `RELEASE.md` | Release Manuals           | Changes to version bumping, tagging, or GitHub Actions draft release workflows.                   |
| `TEST_REPORT.md` / `TESTING.md`  | Testing Policy & Results  | Changes to unit test cases, test environments, execution commands, or verification results.       |
| `PROJECT_TEMPLATE_GUIDE.md`      | Project Template Guide    | Revisions to configuration file templates, CI/CD templates, or automation scripts.                |
| `CONTRIBUTING.md`                | Contributor Guidelines    | Changes to commit conventions, pull request checklists, or workspace setup steps.                 |
| `SECURITY.md`                    | Security Policy           | Changes to security design principles, supported versions, or vulnerability reporting procedures. |
| Source Rustdoc                   | In-Code Documentation     | Synchronize `///` and `//!` comments on code changes. Mandatory when adding types or functions.   |

---

## 5. Quality Assurance, Automated Testing & Pre-Verification Rules

- **Add & Expand Unit Tests**: When adding new features or modifying core logic, add or expand corresponding unit tests.
- **Local Pre-Verification Process** (*Skipped when modifying only `.md` files*):
  1. `cargo test` (All unit tests pass)
  2. `cargo clippy --all-targets -- -D warnings` (Zero Clippy warnings)
  3. `cargo fmt --check` (Full code formatting compliance)
  4. `cargo doc --no-deps --document-private-items` (Zero Rustdoc build warnings)
- **Protect CI/CD & Environment Configs**: Record reasons in `CHANGELOG.md` whenever modifying files under `.github/` or `.editorconfig`.
- **Maintain Shared Library Dependency in GitHub Actions**: Because of the relative dependency on `../common_lib`, preserve the side-by-side checkout of `common_lib` and `working-directory: MiSysMon` in Actions workflows.
- **Skip CI on Markdown-Only Changes**: Preserve the `paths-ignore: '**.md'` setting in CI workflows.
