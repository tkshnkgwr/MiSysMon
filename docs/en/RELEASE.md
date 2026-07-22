**English** | [日本語版](../ja/RELEASE.md)

# Release Manual (RELEASE.md) - MiSysMon

This document outlines the version bumping and release procedures for the `MiSysMon` project.

---

## 1. Pre-Release Preparation

Ensure all code and documentation meet quality standards prior to releasing:

1. **Verification Command Executions**:
   ```bash
   cargo test
   cargo clippy --all-targets -- -D warnings
   cargo fmt --check
   cargo doc --no-deps --document-private-items
   ```
2. **Documentation Check**:
   - Ensure the latest release changes are documented in `docs/ja/CHANGELOG.md` and `docs/en/CHANGELOG.md`.
   - Update performance logs and sizes in `docs/ja/FOOTPRINTS.md` and `docs/en/FOOTPRINTS.md`.

---

## 2. Bumping Versions

1. **Update `Cargo.toml`**:
   ```toml
   [package]
   name = "mini-system-monitor"
   version = "X.Y.Z" # Set new version
   ```
2. **Sync `Cargo.lock`**:
   ```bash
   cargo check
   ```
3. **Update README Badges**:
   - Adjust the release version tag in badge links in `README.md` and `README_JA.md`.
4. **Use Automation Script**:
   - Utilize `./scripts/bump-version.ps1` to update Cargo.toml and documentation versions in a single step:
     ```powershell
     ./scripts/bump-version.ps1 -NewVersion X.Y.Z
     ```

---

## 3. Building and Tagging

1. **Build Optimized Binary**:
   ```bash
   cargo build --release
   ```
2. **Git Commit & Tagging**:
   ```bash
   git add .
   git commit -m "chore(release): bump version to vX.Y.Z"
   git tag -a vX.Y.Z -m "Release version X.Y.Z"
   ```
3. **Push to Remote**:
   ```bash
   git push origin main --tags
   ```

---

## 4. Post-Release Verification

- Check that the GitHub Actions Release workflow finishes successfully, creating a draft release containing the binary (`mini-system-monitor.exe`). Refer to `docs/en/RELEASE_FLOW.md` for details.
