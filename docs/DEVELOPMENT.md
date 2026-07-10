# マルチリポジトリ開発ガイドライン

本プロジェクト（MiSysMon）は、共通処理やOSネイティブAPI依存部分を二重実装しないよう、別のリポジトリである共有ライブラリ `common_lib` に依存しています。
本ドキュメントでは、この構成に基づくローカル開発手順およびGitHub Actionsでの整合性維持について解説します。

---

## 1. 推奨フォルダ構成

ローカル開発環境では、以下のように `MiSysMon` と `common_lib` を**同じ親フォルダの中に並べて配置**してください。

```text
[任意の開発用親ディレクトリ]/
│
├── MiSysMon/              # 本プロジェクト (本リポジトリ)
│   ├── Cargo.toml
│   └── src/
│
└── common_lib/            # 共有ライブラリ (別リポジトリ)
    ├── Cargo.toml
    └── src/
```

---

## 2. Cargo.toml での参照設定

`MiSysMon/Cargo.toml` の `[dependencies]` セクションでは、ローカル開発の利便性を最優先するため、`common_lib` を以下のように**相対パス**で参照しています。

```toml
[dependencies]
# 同一階層の親ディレクトリにある common_lib を参照
common_lib = { path = "../common_lib", features = ["windows_desktop"] }
```

これにより、ローカルで `common_lib` と `MiSysMon` を同時に修正しながら、即座にビルド・テスト・実行確認を行うことができます（リモートへ一旦プッシュするなどの無駄な手数を挟む必要がありません）。

---

## 3. GitHub Actions でのビルド整合性の確保

相対パス `path = "../common_lib"` を使用しているため、GitHub Actions上で `MiSysMon` だけを単純にチェックアウトしてビルドしようとすると、`common_lib` が見つからずにコンパイルエラーになります。

これを防ぐため、GitHub Actionsのワークフロー定義（`.github/workflows/` 以下）では、**「`MiSysMon` の直前の親フォルダに `common_lib` をクローンして並べる」** 設定を標準化しています。

### 具体的なワークフロー構造例 (`ci.yml` / `release.yml` 等)

1. **デフォルト作業ディレクトリの設定**:
   すべての実行ステップが `MiSysMon` の中で動くように、ジョブ全体にデフォルト作業ディレクトリを指定します。
   ```yaml
   defaults:
     run:
       working-directory: MiSysMon
   ```

2. **2つのリポジトリの並行チェックアウト**:
   `actions/checkout` を2回実行し、それぞれの `path` オプションを使って並列に配置します。
   ```yaml
   steps:
     # 1) MiSysMonリポジトリをサブフォルダ「MiSysMon」にクローン
     - name: Checkout MiSysMon
       uses: actions/checkout@v4
       with:
         path: MiSysMon

     # 2) 共通ライブラリをサブフォルダ「common_lib」にクローン
     - name: Checkout common_lib
       uses: actions/checkout@v4
       with:
         repository: tkshnkgwr/common_lib
         path: common_lib
         token: ${{ secrets.PAT || github.token }}
   ```

これにより、Actionsのランナー（仮想環境）のワークスペース内が以下のように構成され、ローカル開発環境と全く同じ相対パスの整合性が維持されます。
- `github.workspace/MiSysMon` (作業ディレクトリ)
- `github.workspace/common_lib` (並行して存在)

---

## 4. 日常の開発・検証手順

コードを変更した際は、コミット・プッシュする前に、本プロジェクトのAI指示書（`AGENTS.md`）およびルールに基づき、必ずローカルで以下の事前検証を行ってください。

1. **フォーマットチェック**:
   ```bash
   cargo fmt --check
   ```
2. **静的解析 (Clippy) の実行**:
   ```bash
   cargo clippy --all-targets -- -D warnings
   ```
3. **ユニットテストの実行**:
   ```bash
   cargo test
   ```

すべてエラー・警告なしで合格したことを確認した上で、変更履歴（`CHANGELOG.md`）を更新してコミット＆プッシュしてください。
