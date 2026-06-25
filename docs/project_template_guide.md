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
├── .gitignore               # ★ルート一本化のGit除外設定
├── Cargo.lock               # (※Cargo.lockはビルド再現性向上のためGitの管理対象にすること)
├── Cargo.toml               # Cargo 設定ファイル
├── CHANGELOG.md             # 更新履歴（CHANGELOG）
├── LICENSE                  # ライセンス
├── README.md                # メイン説明（英語推奨、多言語なら上部リンク）
└── README.ja.md             # 日本語説明（README.mdから分離）
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

## 6. ドキュメント自動更新ルール（AI向け）
AIがコードの変更、機能追加、リファクタリングなどを行う際は、必ず以下のドキュメントをセットで更新または作成すること。

- **`CHANGELOG.md` の自動更新**:
  ソースコードに変更を加えた場合は、作業完了前に必ず変更内容や目的を `CHANGELOG.md` の最新セクションに自動追記すること。
- **`docs/SPEC.md`（仕様書）の更新**:
  新しい機能を追加したり、既存のデータ構造や仕様を変更した場合は、必ず仕様書と整合性をとり、最新の状態に更新すること。
- **`docs/DIAGRAM.md`（システム構成図）の更新**:
  スレッド構造、イベントループ、外部データ取得経路などに変更が生じた場合、Mermaidダイアグラムを更新すること。
- **`README.md` / `README.ja.md` の更新**:
  ビルド・実行手順、依存ライブラリの追加、設定手順に変更があった場合は、必ず英語・日本語双方のドキュメントに反映すること。
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
