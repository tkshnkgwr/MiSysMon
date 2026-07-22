**English** | [日本語版](../ja/DEVELOPMENT.md)

# Multi-Repository Development Guidelines

This project (MiSysMon) depends on a shared library `common_lib` in a separate repository to avoid double implementation of common logic and OS-native API components.
This document explains the local development workflow and how consistency is maintained on GitHub Actions based on this architecture.

---

## 1. Recommended Directory Structure

In your local development environment, place `MiSysMon` and `common_lib` **side-by-side inside the same parent directory** as shown below:

```text
[Any parent development directory]/
│
├── MiSysMon/              # This project (This repository)
│   ├── Cargo.toml
│   └── src/
│
└── common_lib/            # Shared library (Separate repository)
    ├── Cargo.toml
    └── src/
```

---

## 2. Cargo.toml Reference Configuration

In the `[dependencies]` section of `MiSysMon/Cargo.toml`, `common_lib` is referenced using a **relative path** to prioritize local development convenience:

```toml
[dependencies]
# Reference common_lib in the adjacent parent directory
common_lib = { path = "../common_lib", features = ["windows_desktop"] }
```

This configuration allows you to immediately build, test, and run checks while modifying both `common_lib` and `MiSysMon` locally without having to commit and push changes to remote repositories first.

---

## 3. Maintaining Build Consistency in GitHub Actions

Because we use `path = "../common_lib"`, attempting to check out and build only `MiSysMon` on GitHub Actions will result in compilation errors because `common_lib` cannot be found.

To prevent this, the GitHub Actions workflow configurations (under `.github/workflows/`) standardize a configuration that clones `common_lib` into the parent directory of `MiSysMon`.

### Example Workflow Configuration (`ci.yml` / `release.yml`, etc.)

1. **Default Working Directory Setup**:
   Specify a default working directory for the job so that all run steps are executed inside the `MiSysMon` folder.
   ```yaml
   defaults:
     run:
       working-directory: MiSysMon
   ```

2. **Parallel Checkout of Two Repositories**:
   Run `actions/checkout` twice, assigning different values to the `path` option to align them side-by-side.
   ```yaml
   steps:
     # 1) Clone MiSysMon repository to subdirectory "MiSysMon"
     - name: Checkout MiSysMon
       uses: actions/checkout@v4
       with:
         path: MiSysMon

     # 2) Clone the shared library to subdirectory "common_lib"
     - name: Checkout common_lib
       uses: actions/checkout@v4
       with:
         repository: tkshnkgwr/common_lib
         path: common_lib
         token: ${{ secrets.PAT || github.token }}
   ```

This configures the Actions runner's workspace as follows, maintaining the relative path consistency exactly like your local development environment:
- `github.workspace/MiSysMon` (working directory)
- `github.workspace/common_lib` (exists side-by-side)

---

## 4. Daily Development & Verification Procedures

When making changes, always run the following local verifications prior to committing or pushing, in accordance with the AI Agent rules (`AGENTS.md`):

1. **Format Check**:
   ```bash
   cargo fmt --check
   ```
2. **Static Analysis (Clippy)**:
   ```bash
   cargo clippy --all-targets -- -D warnings
   ```
3. **Unit Tests**:
   ```bash
   cargo test
   ```

After confirming that all checks pass with zero warnings and errors, update the changelog (`CHANGELOG.md`) and proceed to commit and push.
