**English** | [日本語版](../ja/INSTRUCTIONS.md)

# AI Instructions: Coding Standards & Guidelines

This document defines the coding standards, conventions, and output formats that the AI Agent (Daikenja) must adhere to when proposing modifications, fixes, or additions to the source code of this project.

---

## 1. Naming Conventions

Strictly adhere to the standard Rust API Guidelines (RFC 430).

- **Types, Structs, Enums, and Traits**:
  Use `UpperCamelCase` (PascalCase).
  - *Examples*: `SystemMonitor`, `Config`, `NetworkStats`
- **Functions, Methods, Variables, Macros, and Modules**:
  Use `snake_case`.
  - *Examples*: `update_stats`, `cpu_usage`, `net_down`
- **Constants & Static Variables**:
  Use `SCREAMING_SNAKE_CASE`.
  - *Example*: `const UPDATE_INTERVAL: Duration = ...`
- **Filenames**:
  - Source code and scripts: `snake_case.rs`
  - Documentation files (Markdown, etc.): **SCREAMING_SNAKE_CASE (`SCREAMING_SNAKE_CASE.md`)**
    - *Examples*: `SPEC.md`, `ARCHITECTURE.md`, `INSTRUCTIONS.md`, `TODO.md`, `README_JA.md`

---

## 2. Error Handling

Write robust code that preserves Rust's safety guarantees.

- **Abolishing `panic!` and `unwrap()`**:
  To prevent application panics and unexpected crashes, avoid `.unwrap()` and `.expect()` in non-test code.
- **Error Propagation via Return Types**:
  Return `Result<T, E>` or `Option<T>` for fallible operations, allowing callers to handle errors properly.
- **Safe Defaults and Fallback Logic**:
  - If retrieving hardware info (such as temperature) fails or if running on unsupported OSes, return default values (e.g. `0.0`) or implement safe fallback logic (e.g., take the maximum temperature among available sensors).
  - *Examples*: Utilize `unwrap_or(0.0)`, `unwrap_or_default()`, etc.
- **Error Handling on File I/O and Config Load**:
  When receiving `Result`, use `if let Ok(...)` or `map_or` to protect the application from crashing.

---

## 3. Component & Module Division Criteria

- **Maintaining Single Source File**:
  MiSysMon prioritizes minimal resource footprint and simplicity. Currently, it consists of only `src/main.rs`. Propose new minor helper features inside `src/main.rs`.
- **Trigger for Module Division**:
  - If a single source file (e.g., `src/main.rs`) exceeds **1,000 lines**, or if a major component (like `eframe::App` implementation or `SystemMonitor` metrics collection logic) exceeds 500 lines, the AI Agent (Daikenja) must proactively propose module splitting (e.g., into `src/gui.rs` and `src/monitor.rs`) and refactoring.
- **Utilizing Shared Crate (`common_lib`)**:
  - Low-level processes that can be shared among other desktop applications (such as double-launch mutex guards or byte formatting logic) must be imported from the local shared crate `common_lib` located in the parent directory, rather than implementing them inside this repository.

---

## 4. AI Response and Output Formats

When presenting solutions to the user, adhere to the following rules:

- **Keep Explanations Minimal**:
  Omit unnecessary explanations or redundant text that can easily be understood by reading the code or docs. Focus on presenting conclusions and diff blocks.
- **Format for Proposing Code Changes**:
  - For contiguous changes, clearly state target/replacement code snippets inside diff blocks so `replace_file_content` can be easily applied.
  - Do not reprint the entire source code file.
- **Automatic Documentation**:
  When modifying source code, search and update relevant documentation files (`CHANGELOG.md`, `SPEC.md`, `ARCHITECTURE.md`, etc.) automatically.
