# Rust デスクトップアプリ開発：新規プロジェクト用雛形ガイドライン

Google AI Studio や AntiGravity（Gem）と共同で Rust デスクトップアプリ（eframe/egui等）を開発する際、初期設定や文字コード問題で躓かないための「黄金構成」と「AI指示書（AGENTS.md）」のテンプレートです。

次回以降のプロジェクト作成時は、この構成をコピー＆ペーストすることで、安全かつ綺麗な状態でスムーズに開発をスタートできます。

---

## 1. 推奨するフォルダ・ファイル構成 (Directory Structure)

リポジトリルートには余計なファイルを置かず、AIが作業しやすいように分類します。

```text
[project-root]/
│
├── .agents/                 # AI(Gem)向け指示書の格納先
│   └── AGENTS.md            # ★AI用ルールブック（下記テンプレートを使用）
│
├── .github/                 # ★GitHub 連携設定
│   ├── dependabot.yml       # ★依存関係自動更新設定（毎週チェック）
│   └── workflows/
│       ├── ci.yml           # ★CIワークフロー（ビルド・テスト・fmt・clippy自動検証）
│       ├── release.yml      # ★自動リリースワークフロー（タグプッシュ時に自動ドラフト作成）
│       └── bump-version.yml # ★自動バージョンアップワークフロー（mainプッシュ時にタグ自動作成）
│
├── docs/                    # 各種技術ドキュメント（ルートを汚さないため）
│   ├── SPEC.md              # ★製品仕様書（バージョン・技術スタック記載）
│   ├── DIAGRAM.md           # ★システム構成・データフロー図（Mermaid）
│   ├── FOOTPRINTS.md        # ★パフォーマンス・リソース使用量計測記録
│   └── TEST_REPORT.md       # ★テストレポート（検証結果記載）
│
├── scripts/                 # 自動化スクリプト
│   └── bump-version.ps1     # ★文字コード対策済・バージョン一括更新スクリプト
│
├── src/                     # Rust ソースコード（必要に応じてサブモジュールに分割）
│   └── main.rs
│
├── .editorconfig            # ★エディタ間スタイル統一設定
├── .gitignore               # ★ルート一本化のGit除外設定
├── Cargo.lock               # (※Cargo.lockはビルド再現性向上のためGitの管理対象にすること)
├── Cargo.toml               # Cargo 設定ファイル
├── CHANGELOG.md             # 更新履歴（CHANGELOG）
├── LICENSE                  # ライセンス
├── README.md                # メイン説明（英語推奨、多言語なら上部リンク）
└── README_JA.md             # 日本語説明（README.mdから分離）
```

---

## 2. 新規プロジェクト用 `.gitignore` テンプレート

Cargo 関連および OS・エディタ関連の除外設定をルートに一本化したものです。ビルドの再現性を保つため、`Cargo.lock` は除外**しません**。

```gitignore
# Rust (Cargo) 関連の除外
/target/
**/*.rs.bk
*.pdb

# OS・エディタ関連の除外
.DS_Store
Thumbs.db
/.vscode/
/.idea/

# エディタの一時ファイル・バックアップファイル
*.un~
*~
*.swp
```

---

## 3. ひな形 `.agents/AGENTS.md` (AI向けカスタムルール)

プロジェクト作成時にこれを `.agents/AGENTS.md` に配置することで、AIがRustデスクトップアプリの特性や各種ドキュメントの自動更新ルールを最初から理解して動くようになります。

```markdown
# Rust デスクトップアプリ開発ガイドライン

このプロジェクトでデスクトップアプリ（eframe/egui等）を開発する際は、以下の「黄金設定」を厳守すること。

## 1. ウィンドウの影と枠を完全に消す (Windows/eframe)
透過ウィンドウで「薄い枠」や「影」が出るのを防ぐため、以下の設定をセットで行う。

- **eframe::NativeOptions**:
  `decorated: false`, `transparent: true` を指定。
  Windows API を呼ぶ場合は、ウィンドウ作成時に影を完全にオフにする処理を追加する。

## 2. ドラッグ操作とイベント透過の制御
タイトルバーを非表示（decorated: false）にするため、カスタムUI上でウィンドウをドラッグ移動できるようにする。
- **egui**: ドラッグ用のグリップ領域やヘッダーを設け、そこがドラッグされた際に `ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag)` を呼び出してネイティブのドラッグ移動を開始させる。

## 3. 二重起動の防止 (Windows Named Mutex)
Windows上での多重起動を防ぐため、`windows` クレート等の Named Mutex を用いた二重起動防止ロジックを `fn main()` の最初で実行する。既に起動している場合は、新しく起動されたプロセスが即座に正常終了するように実装すること。

## 4. リリース時のサイズ最適化
バイナリサイズとメモリフットプリントを最小限にするため、`Cargo.toml` の `[profile.release]` に以下の最適化設定を適用する：
- `opt-level = 'z'` (サイズ優先の最適化)
- `lto = true` (リンク時最適化)
- `codegen-units = 1` (コンパイル単位の統合)
- `panic = 'abort'` (巻き戻し無効化)
- `strip = true` (シンボル情報の削除)

## 5. 開発および実行時のルール (Windows/PowerShell)
- **PowerShell 7 (pwsh) の使用**: 
  コマンド実行時は、古い Windows PowerShell (5.1) ではなく、必ず最新の `pwsh` を使用して文字コード問題を回避すること。
- **BOMなし UTF-8 の徹底**:
  ファイルはすべて BOMなし UTF-8 で読み書きすること。
- **ターミナルの文字化け対策**:
  Windows環境で `cargo run` などの出力を表示する際、日本語が文字化けする場合は、ターミナルのエンコーディングを UTF-8 (`chcp 65001` または `[Console]::OutputEncoding = [System.Text.Encoding]::UTF8`) に設定すること。

## 6. 品質管理・自動テストと検証ルール（AI向け）
- **テストコードの追加・拡充**:
  新しい機能を追加した際、または既存ロジックを変更した際は、可能な限り対応するユニットテストを実装または拡張すること。
- **ローカルでの事前検証**:
  コミット前、あるいはタスクの完了報告前に、必ずローカル環境で以下のチェックを行い、すべて合格することを確認すること：
  * `cargo test` (自動テストがすべてグリーンであること)
  * `cargo clippy --all-targets -- -D warnings` (警告やエラーが一切ないこと)
  * `cargo fmt --check` (コードフォーマットが完全に準拠していること)
- **CI/CD設定およびエディタ設定の保護**:
  `.github/workflows/` 内のワークフロー定義、`.github/dependabot.yml`、`.editorconfig` などのシステム設定を変更した場合は、必ずその変更理由と内容を `CHANGELOG.md` に記録すること。

## 7. ドキュメント自動更新ルール（AI向け）
AIがコードの変更、機能追加、リファクタリングなどを行う際は、必ず以下のドキュメントをセットで更新または作成すること。

- **`CHANGELOG.md` の自動更新**:
  ソースコードに変更を加えた場合は、作業完了前に必ず変更内容や目的を `CHANGELOG.md` の最新セクションに自動追記すること。
- **`docs/SPEC.md`（仕様書）の更新**:
  新しい機能を追加したり、既存のデータ構造や仕様を変更した場合は、必ず仕様書と整合性をとり、最新の状態に更新すること。
- **`docs/DIAGRAM.md`（システム構成図）の更新**:
  スレッド構造、イベントループ、外部データ取得経路などに変更が生じた場合、Mermaidダイアグラムを更新すること。
- **`README.md` / `README_JA.md` の更新**:
  ビルド・実行手順、依存ライブラリの追加、設定手順に変更があった場合は、必ず英語・日本語双方のドキュメントに反映すること。
  また、CI の導入後はヘッダーにビルドステータスバッジを追加・維持すること。
- **`docs/FOOTPRINTS.md` の更新**:
  リリースビルドのバイナリサイズや、メモリ使用量の大幅な増減があった場合、または新たな計測結果が得られた場合は、計測値を本ドキュメントに記録・更新すること。
- **`docs/TEST_REPORT.md` の更新**:
  機能実装時やリファクタリング時にテスト（自動テスト・手動テスト）を実行した場合は、検証手順や結果を記録・更新すること。
- **ドキュメントの整合性チェック**:
  タスク完了時には、コードと各種Markdownドキュメントの間に情報のズレが残っていないか必ずセルフチェックすること。
```

---

## 4. 汎用バージョン更新スクリプト `scripts/bump-version.ps1`

PowerShell 5.1/7 の双方で動作し、絶対に文字化けやBOMエラーを発生させずにバージョンを一括更新するスクリプトです。

新規プロジェクト作成時は、この内容を `scripts/bump-version.ps1` として作成し、BOMなしUTF-8（アスキー文字列に日本語コードキャストを併用した文字化け回避策）で保存してください。

```powershell
param (
    [Parameter(Mandatory=$true)]
    [string]$NewVersion
)

# セマンティックバージョン (X.Y.Z) のバリデーション
if ($NewVersion -notmatch '^\d+\.\d+\.\d+$') {
    Write-Error "Error: Version must be in semantic versioning format (e.g. 1.2.3)"
    exit 1
}

$InternalVersion = "$NewVersion.0"

Write-Host "Bumping version to $NewVersion (Internal: $InternalVersion)..."
# BOMなしUTF-8を生成するための .NET エンコーディングオブジェクト
$utf8 = New-Object System.Text.UTF8Encoding($false)

# PowerShell 5.1 のスクリプト文字化けを回避するため、日本語文字列を文字コードから動的生成
$versionLabel  = [char]0x30d0 + [char]0x30fc + [char]0x30b8 + [char]0x30e7 + [char]0x30f3          # "バージョン"
$internalLabel = [char]0x5185 + [char]0x90e8 + $versionLabel                                     # "内部バージョン"
$suitableLabel = [char]0x9069 + [char]0x5408 + $versionLabel                                     # "適合バージョン"

# 1. Cargo.toml の更新 (行配列処理)
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

# 2. docs/SPEC.md の更新 (バージョンおよび内部バージョンの動的置換・挿入)
$specPath = "docs/SPEC.md"
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

# 3. docs/TEST_REPORT.md の更新
$testPath = "docs/TEST_REPORT.md"
if (Test-Path $testPath) {
    Write-Host "Updating $testPath..."
    $lines = [System.IO.File]::ReadAllLines($testPath, $utf8)
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match "\*\*$suitableLabel\*\*") {
            # プロジェクトに合わせて適宜書き換え
            $lines[$i] = "**$suitableLabel**: mini-system-monitor v$NewVersion"
            break
        }
    }
    [System.IO.File]::WriteAllLines($testPath, $lines, $utf8)
} else {
    Write-Warning "$testPath not found."
}

Write-Host "Version bump completed successfully!"
```

---

## 5. CI/CD・自動テスト・パッケージ管理の黄金設定

新規プロジェクトで「自動テスト」「自動ビルド」「自動リリース」「依存関係自動監視」を標準化するために導入する設定ファイルのテンプレートです。

### 5.1. CI ワークフロー: `.github/workflows/ci.yml`
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

### 5.2. 自動リリースワークフロー: `.github/workflows/release.yml`
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
          # 必要に応じて、ビルドされたバイナリ名をプロジェクトのものに変更する
          files: target/release/mini-system-monitor.exe
```

### 5.3. 自動バージョンアップワークフロー: `.github/workflows/bump-version.yml`
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

### 5.4. Dependabot 設定: `.github/dependabot.yml`
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

### 5.5. エディタ共通設定: `.editorconfig`
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
```
