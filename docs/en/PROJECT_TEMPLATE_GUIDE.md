**English** | [日本語版](../ja/PROJECT_TEMPLATE_GUIDE.md)

# Rust Desktop Application Development: New Project Template Guidelines

This is a template for the "Golden Configuration" and "AI Instructions (AGENTS.md)" to avoid stumbling on initial settings or character encoding issues when developing Rust desktop applications (eframe/egui, etc.) in collaboration with Google AI Studio or AntiGravity (Gem).

For future project creation, copying and pasting this structure will ensure a clean, safe, and smooth start for development.

---

## 1. Recommended Directory Structure

Keep the repository root clean and organize files so the AI can easily navigate the workspace.

```text
[project-root]/
│
├── .agents/                 # AI (Daikenja) Instruction Storage
│   └── AGENTS.md            # ★ AI Rule Book (using template below)
│
├── .github/                 # ★ GitHub Integration Settings
│   ├── dependabot.yml       # ★ Auto-dependency updates (weekly checks)
│   └── workflows/
│       ├── ci.yml           # ★ CI Workflow (auto verification of build, test, fmt, clippy)
│       ├── release.yml      # ★ Auto-release Workflow (auto draft creation on tag push)
│       └── bump-version.yml # ★ Auto-version bump Workflow (auto tag creation on main push)
│
├── docs/                    # Technical Documents (to avoid polluting root)
│   ├── en/                  # English Documents
│   │   ├── SPEC.md          # ★ Product Specifications
│   │   ├── DIAGRAM.md       # ★ System & Dataflow Diagram (Mermaid)
│   │   ├── FOOTPRINTS.md    # ★ Performance/Resource metrics logs
│   │   ├── INSTRUCTIONS.md  # ★ AI Coding Standards
│   │   ├── TODO.md          # ★ Project Tasks
│   │   └── TEST_REPORT.md   # ★ Test Report
│   └── ja/                  # Japanese Documents
│       ├── SPEC.md          
│       ├── DIAGRAM.md       
│       ├── FOOTPRINTS.md    
│       ├── INSTRUCTIONS.md  
│       ├── TODO.md          
│       └── TEST_REPORT.md   
│
├── scripts/                 # Automation Scripts
│   └── bump-version.ps1     # ★ Version update script handling character encodings
│
├── src/                     # Rust Source Code (split into submodules if necessary)
│   └── main.rs
│
├── .editorconfig            # ★ Style unification settings across editors
├── .gitignore               # ★ Root Git ignore rules
├── Cargo.lock               # (Keep Cargo.lock in Git for reproducible builds!)
├── Cargo.toml               # Cargo Config
├── LICENSE                  # MIT or other License
├── README.md                # Main description (English, with language links at top)
└── README_JA.md             # Japanese description (separated from README.md)
```

---

## 2. `.gitignore` Template for New Projects

Consolidates Cargo, OS, and editor ignore rules into the root. `Cargo.lock` is **not** ignored to maintain build reproducibility.

```gitignore
# Rust (Cargo) Exclusions
/target/
**/*.rs.bk
*.pdb

# OS/Editor Exclusions
.DS_Store
Thumbs.db
/.vscode/
/.idea/

# Editor Temp & Backup Files
*.un~
*~
*.swp
```

---

## 3. Template `.agents/AGENTS.md` (Custom Rules for AI)

Placing this under `.agents/AGENTS.md` at project initialization allows the AI to understand the characteristics of Rust desktop apps and automatic document updates from the start.

```markdown
# Rust Desktop Application Development Guidelines

When developing desktop applications (eframe/egui, etc.) in this project, strictly follow these "Golden Settings".

## 1. Eliminate Window Shadows and Borders (Windows/eframe)
To prevent "thin borders" or "shadows" on transparent windows, set the following options together:
- **eframe::NativeOptions**: Specify `decorated: false`, `transparent: true`. If calling Windows APIs, add logic to turn off shadows completely at window creation.

## 2. Drag Operations and Event Pass-through Control
To allow moving the window when decorated: false, implement custom UI dragging.
- **egui**: Provide a grip region or header and call `ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag)` when dragged to start native dragging.

## 3. Double-Launch Prevention (Windows Named Mutex)
Implement double-launch prevention logic at the start of `fn main()` using Named Mutex. Ensure that if an instance is already running, the new process terminates immediately and normally.

## 4. Size Optimization for Release
To minimize binary size and memory footprint, apply the following optimization profiles in `Cargo.toml` under `[profile.release]`:
- `opt-level = 'z'` (optimize for size)
- `lto = true` (link-time optimization)
- `codegen-units = 1` (consolidate compile units)
- `panic = 'abort'` (disable unwinding)
- `strip = true` (strip symbols)

## 5. Development and Execution Rules (Windows/PowerShell)
- **Use PowerShell 7 (pwsh)**: Always run commands using the latest `pwsh` instead of legacy Windows PowerShell (5.1) to avoid encoding issues.
- **Strictly No BOM UTF-8**: Read and write files using UTF-8 without BOM.
- **Terminal Encoding**: If Japanese/special characters are garbled in the terminal on Windows, configure it to UTF-8 (`chcp 65001` or `[Console]::OutputEncoding = [System.Text.Encoding]::UTF8`).

## 6. Quality Control, Automated Testing & Verification Rules (for AI)
- **Add & Extend Tests**: Implement or extend corresponding unit tests whenever adding new features or modifying existing logic.
- **Local Verification**: Prior to committing or reporting tasks as complete, ensure that:
  * `cargo test` passes successfully.
  * `cargo clippy --all-targets -- -D warnings` reports zero warnings/errors.
  * `cargo fmt --check` complies with standard formatting.
- **Protect CI/CD and Editor Configurations**: If modifying workflows under `.github/workflows/`, `.github/dependabot.yml`, or `.editorconfig`, record the details in `CHANGELOG.md`.

## 7. Auto-Documentation Rules (for AI)
Whenever modifying code, adding features, or refactoring, update or create the following documents simultaneously:
- **Auto-update `CHANGELOG.md`**: Append changes to the latest section of `CHANGELOG.md` before completing tasks.
- **Update `docs/en/*/SPEC.md`**: Synchronize specifications with implementation.
- **Update `docs/en/*/DIAGRAM.md`**: Update Mermaid diagrams if thread structures, event loops, or data acquisition channels are changed.
- **Update `README.md` / `README_JA.md`**: Update build instructions, dependencies, and configuration steps in both English and Japanese.
- **Update `docs/en/*/FOOTPRINTS.md`**: Record metrics if release binary sizes or memory footprints change.
- **Update `docs/en/*/TEST_REPORT.md`**: Log verification steps and results.
- **Consistency Check**: Perform a self-check to ensure there are no discrepancies between the code and Markdown documents.
```

---

## 4. Universal Version Bump Script `scripts/bump-version.ps1`

Works on both PowerShell 5.1/7 to bump versions across files without character encoding errors or BOM issues. Save as `scripts/bump-version.ps1` in UTF-8 without BOM.

```powershell
param (
    [Parameter(Mandatory=$true)]
    [string]$NewVersion
)

# Validate semantic versioning format (X.Y.Z)
if ($NewVersion -notmatch '^\d+\.\d+\.\d+$') {
    Write-Error "Error: Version must be in semantic versioning format (e.g. 1.2.3)"
    exit 1
}

$InternalVersion = "$NewVersion.0"

Write-Host "Bumping version to $NewVersion (Internal: $InternalVersion)..."
# UTF-8 without BOM encoding object
$utf8 = New-Object System.Text.UTF8Encoding($false)

# Dynamic generation of Japanese strings from char codes to avoid encoding bugs on PowerShell 5.1
$versionLabel  = [char]0x30d0 + [char]0x30fc + [char]0x30b8 + [char]0x30e7 + [char]0x30f3          # "バージョン"
$internalLabel = [char]0x5185 + [char]0x90e8 + $versionLabel                                     # "内部バージョン"
$suitableLabel = [char]0x9069 + [char]0x5408 + $versionLabel                                     # "適合バージョン"

# 1. Update Cargo.toml
$cargoPath = "Cargo.toml"
if (Test-Path $cargoPath) {
    Write-Host "Updating $cargoPath..."
    $lines = [System.IO.File]::ReadAllLines($cargoPath, $utf8)
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match '^version\s*=\s*"[^"]+"') {
            $lines[$i] = "version = `"$NewVersion`""
            break
        }
    }
    [System.IO.File]::WriteAllLines($cargoPath, $lines, $utf8)
} else {
    Write-Warning "$cargoPath not found."
}

# 2. Update docs/ja/SPEC.md and docs/en/SPEC.md
$specPaths = @("docs/ja/SPEC.md", "docs/en/SPEC.md")
foreach ($specPath in $specPaths) {
    if (Test-Path $specPath) {
        Write-Host "Updating $specPath..."
        $lines = [System.IO.File]::ReadAllLines($specPath, $utf8)
        $newLines = [System.Collections.Generic.List[string]]::new()
        
        $hasInternal = $false
        for ($i = 0; $i -lt $lines.Length; $i++) {
            if ($lines[$i] -match "\*\*$internalLabel\*\*") {
                $hasInternal = $true
                break
            }
        }
        
        for ($i = 0; $i -lt $lines.Length; $i++) {
            $line = $lines[$i]
            if ($line -match "\*\*$versionLabel\*\*") {
                $line = "**$versionLabel**: $NewVersion"
                $newLines.Add($line)
                if (-not $hasInternal) {
                    $newLines.Add("**$internalLabel**: $InternalVersion")
                }
            } elseif ($line -match "\*\*$internalLabel\*\*") {
                $line = "**$internalLabel**: $InternalVersion"
                $newLines.Add($line)
            } else {
                $newLines.Add($line)
            }
        }
        [System.IO.File]::WriteAllLines($specPath, $newLines.ToArray(), $utf8)
    } else {
        Write-Warning "$specPath not found."
    }
}

# 3. Update docs/ja/TEST_REPORT.md and docs/en/TEST_REPORT.md
$testPaths = @("docs/ja/TEST_REPORT.md", "docs/en/TEST_REPORT.md")
foreach ($testPath in $testPaths) {
    if (Test-Path $testPath) {
        Write-Host "Updating $testPath..."
        $lines = [System.IO.File]::ReadAllLines($testPath, $utf8)
        for ($i = 0; $i -lt $lines.Length; $i++) {
            if ($lines[$i] -match "\*\*$suitableLabel\*\*") {
                $lines[$i] = "**$suitableLabel**: mini-system-monitor v$NewVersion"
                break
            }
        }
        [System.IO.File]::WriteAllLines($testPath, $lines, $utf8)
    } else {
        Write-Warning "$testPath not found."
    }
}

Write-Host "Version bump completed successfully!"
```

---

## 5. Golden Configs for CI/CD, Testing, and Package Management

Templates for automated workflows, checks, and release management in new projects.

### 5.1. CI Workflow: `.github/workflows/ci.yml`
```yaml
name: CI

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Build & Test
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install dependencies (Ubuntu only)
        if: matrix.os == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev libgtk-3-dev

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - name: Enable Rust cache
        uses: swatinem/rust-cache@v2

      - name: Check formatting
        run: cargo fmt --all -- --check

      - name: Lint with Clippy
        run: cargo clippy --all-targets -- -D warnings

      - name: Run tests
        run: cargo test --all-targets
```

### 5.2. Automated Release Workflow: `.github/workflows/release.yml`
```yaml
name: Release

on:
  push:
    tags:
      - "v*"

jobs:
  release:
    name: Create Draft Release
    runs-on: windows-latest
    permissions:
      contents: write

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Enable Rust cache
        uses: swatinem/rust-cache@v2

      - name: Build release binary
        run: cargo build --release

      - name: Create Draft Release
        uses: softprops/action-gh-release@v2
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          draft: true
          # Replace binary name as appropriate for the target project
          files: target/release/mini-system-monitor.exe
```

### 5.3. Automated Version Bump Workflow: `.github/workflows/bump-version.yml`
```yaml
name: Auto Bump Version

on:
  push:
    branches:
      - main

jobs:
  bump:
    name: Bump Version & Tag
    runs-on: windows-latest
    if: "!contains(github.event.head_commit.message, '[skip ci]')"
    permissions:
      contents: write

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Calculate new version
        id: calculate_version
        shell: powershell
        run: |
          # Read Cargo.toml and find the current version
          $cargo = Get-Content Cargo.toml
          $versionLine = $cargo | Select-String -Pattern '^version\s*=\s*"(.*)"'
          if (-not $versionLine) {
              Write-Error "Could not find version in Cargo.toml"
              exit 1
          }
          $currentVersion = $versionLine.Matches.Groups[1].Value
          Write-Host "Current version: $currentVersion"

          # Increment the patch version (X.Y.Z -> X.Y.Z+1)
          if ($currentVersion -match '^(\d+)\.(\d+)\.(\d+)$') {
              $major = [int]$Matches[1]
              $minor = [int]$Matches[2]
              $patch = [int]$Matches[3]
              $newPatch = $patch + 1
              $newVersion = "$major.$minor.$newPatch"
              Write-Host "New version: $newVersion"
              echo "NEW_VERSION=$newVersion" >> $env:GITHUB_ENV
          } else {
              Write-Error "Invalid version format: $currentVersion"
              exit 1
          }

      - name: Run version bump script
        shell: powershell
        run: |
          ./scripts/bump-version.ps1 -NewVersion ${{ env.NEW_VERSION }}

      - name: Commit and Push changes
        shell: powershell
        run: |
          git config --local user.email "github-actions[bot]@users.noreply.github.com"
          git config --local user.name "github-actions[bot]"
          git add .
          git commit -m "chore(release): bump version to v${{ env.NEW_VERSION }} [skip ci]"
          git push origin main

      - name: Create and Push Tag
        shell: powershell
        run: |
          git tag "v${{ env.NEW_VERSION }}"
          git push origin "v${{ env.NEW_VERSION }}"
```

### 5.4. Dependabot Config: `.github/dependabot.yml`
```yaml
version: 2
updates:
  # Maintain dependencies for Cargo
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"

  # Maintain dependencies for GitHub Actions
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
```

### 5.5. EditorConfig: `.editorconfig`
```editorconfig
root = true

[*]
end_of_line = lf
charset = utf-8
trim_trailing_whitespace = true
insert_final_newline = true
indent_style = space
indent_size = 4

[*.rs]
indent_style = space
indent_size = 4

[*.yml]
indent_style = space
indent_size = 2

[*.md]
trim_trailing_whitespace = false
indent_style = space
indent_size = 4
```
