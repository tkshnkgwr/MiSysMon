[English](../en/TESTING.md) | **日本語版**

# テスト方針・実行ガイド (TESTING.md) - MiSysMon

本文書は、`MiSysMon` プロジェクトにおけるユニットテスト、品質検証に関する方針、モジュール別テスト観点、および実行手順をまとめたドキュメントです。

---

## 1. テスト概要

`MiSysMon` は Rust, egui, eframe, sysinfo 等を使用した軽量なシステムモニターであり、高い信頼性と堅牢性を維持するために包括的な検証を行います。

主要なテスト観点：
- **設定の永続化と後方互換性 (`config`)**: `serde_json` による完全なシリアライズ/デシリアライズ往復テスト、未知または一部欠落した設定 JSON に対する安全なデフォルト補完テスト。
- **画面端オートスナップと境界値 (`win32`)**: 上下左右の画面端（しきい値 15px 以内）での吸着、しきい値外での非吸着、および四隅コーナーでの同時吸着計算の正当性検証。
- **UIレイアウトと動的幅計算 (`ui`)**: 全項目表示時、全項目非表示時（最小幅 200px クランプ）、および任意の組み合わせ時におけるウィンドウ幅の動的計算テスト。
- **メトリクス管理とログ世代交代 (`monitor`)**: スパークライン用履歴キューの最大件数（30件）制限動作、ログファイルの最大 3 世代ローテーション処理、およびネットワーク・ディスク I/O 差分計算の飽和演算（`saturating_sub`）テスト。
- **二重起動防止**: 共有ライブラリ `common_lib` を通じた Named Mutex による二重起動制限の挙動。

---

## 2. モジュール別ユニットテスト一覧

現在実装されている 15 件のユニットテスト構成：

| モジュール   | テスト関数名                                                    | 検証内容                                                               |
| :----------- | :-------------------------------------------------------------- | :--------------------------------------------------------------------- |
| `config`     | `test_config_default_values`                                    | デフォルト設定値の整合性（全表示ON、1.0s更新、不透明度0.78等）         |
| `config`     | `test_config_json_serialization_roundtrip`                      | JSON シリアライズおよびデシリアライズの完全往復整合性                  |
| `config`     | `test_config_json_partial_fallback`                             | 一部キー欠落時の安全なデフォルト値フォールバック                       |
| `win32`      | `test_apply_snap_left_edge`                                     | 左端（15px以内）への吸着動作および負の座標境界値                       |
| `win32`      | `test_apply_snap_right_edge`                                    | 右端（15px以内）への吸着動作                                           |
| `win32`      | `test_apply_snap_top_edge`                                      | 上端（15px以内）への吸着動作                                           |
| `win32`      | `test_apply_snap_bottom_edge`                                   | 下端（15px以内）への吸着動作                                           |
| `win32`      | `test_apply_snap_no_snap_when_outside_threshold`                | しきい値（15px）を超える位置での非吸着動作                             |
| `win32`      | `test_apply_snap_corner_snap`                                   | 左上隅コーナーへの同時吸着動作                                         |
| `ui`         | `test_calculate_window_width_all_enabled`                       | 全項目表示時の動的幅計算（1210px）                                     |
| `ui`         | `test_calculate_window_width_all_disabled_minimum_clamping`     | 全項目非表示時の最小幅制限（200px）                                    |
| `ui`         | `test_calculate_window_width_custom_selection`                  | 任意選択時の正確な幅計算（CPU + Clock = 450px）                        |
| `monitor`    | `test_push_history_capacity`                                    | スパークライン用履歴キューの最大長（30件）スライド動作                 |
| `monitor`    | `test_rotate_logs_succession`                                   | ログローテーションによる `.1`, `.2` への世代シフト動作                  |
| `monitor`    | `test_saturating_diff_calculation`                              | 累積通信量差分計算およびアンダーフロー時のゼロ安全保護 (`saturating`)  |

---

## 3. テストの実行手順

ローカル環境でのテスト実行手順は以下の通りです。

### ユニットテストの実行
```bash
cargo test
```

### 品質・コード検証コマンド
コード変更完了時には、以下の標準コマンドが全てエラー・警告なしで合格することを確認します：

```bash
# 1. ユニットテスト実行
cargo test

# 2. 静的解析 (Clippy)
cargo clippy --all-targets -- -D warnings

# 3. コードフォーマット検証
cargo fmt --check

# 4. Rustdoc ビルド検証
cargo doc --no-deps --document-private-items
```

---

## 4. テストの記述ガイドライン

1. **モジュール内テスト**:
   - 各機能モジュール（`src/*.rs`）内の `#[cfg(test)] mod tests` にユニットテストを記述し、関心事の近い場所で検証を完結させます。
2. **パニック回避・フォールバックのテスト**:
   - センサー取得の失敗や OS 依存処理、JSON の欠損データに起因してアプリケーション全体がクラッシュしないためのフォールバック処理を徹底検証します。
