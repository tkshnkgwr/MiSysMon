[English](../en/CONTRIBUTING.md) | **日本語版**

# 貢献ガイドライン (CONTRIBUTING.md) - MiSysMon

`MiSysMon` プロジェクトへの貢献に興味を持っていただきありがとうございます！
本文書では、バグ報告、機能提案、プルリクエスト提出時のガイドラインを説明します。

---

## 1. 開発方針と重要原則

開発を行う際は、以下の基本方針を遵守してください。

1. **低リソース・安全・軽量へのこだわり**:
   - 不要なサードパーティクレートを増やさないでください。また、システムポーリングの負荷を抑え、バイナリサイズ削減に配慮した設計を維持します。
2. **共有ライブラリの活用 (`common_lib`)**:
   - ウィンドウ制御（二重起動防止）やバイトフォーマット処理など、他プロジェクトと共有可能な共通処理は `common_lib` 側に配置し、MiSysMon 本体コードの肥大化を防ぎます。
3. **多言語ドキュメントの同期**:
   - 仕様変更や機能追加を行う際は、`docs/ja/` および `docs/en/` の両方のドキュメントを更新し、整合性を維持してください。

---

## 2. 開発環境のセットアップ

1. **親ディレクトリに `common_lib` をクローン（並列配置）**:
   ```bash
   git clone https://github.com/tkshnkgwr/common_lib.git
   ```
2. **`MiSysMon` のクローン**:
   ```bash
   git clone https://github.com/tkshnkgwr/MiSysMon.git
   cd MiSysMon
   ```
3. **動作確認**:
   ```bash
   cargo run
   ```

---

## 3. コミットおよびプルリクエスト手順

### コミットメッセージの規約
コミットメッセージには Conventional Commits 形式を使用してください：

- `feat:` 新機能の追加
- `fix:` バグ修正
- `docs:` ドキュメントの変更
- `refactor:` リファクタリング
- `perf:` パフォーマンス改善
- `test:` テストの追加・修正
- `chore:` ビルドスクリプトや設定の変更

### プルリクエスト作成前のチェックリスト
プルリクエストを送信する前に、以下のコマンドを実行し全て合格することを確認してください：

- [ ] `cargo test` （ユニットテスト合格）
- [ ] `cargo clippy --all-targets -- -D warnings` （静的解析の警告ゼロ）
- [ ] `cargo fmt --check` （コードフォーマット準拠）
- [ ] `cargo doc --no-deps --document-private-items` （ドキュメントビルドエラーゼロ）
