[English](../en/TEST_REPORT.md) | **日本語版**

# Test Report: Mini System Monitor (MiSysMon)

**適合バージョン**: mini-system-monitor v1.0.0

## 1. テスト環境
- **OS:** Windows 11 / Windows 10
- **Rust Toolchain:** 1.80+ (stable-x86_64-pc-windows-msvc)
- **主要クレート:** eframe 0.35.0, sysinfo 0.38.0, chrono 0.4.38, serde 1.0.203, common_lib 0.2.4

---

## 2. ユニットテストおよび機能検証結果

全 15 件の自動ユニットテストおよび機能検証を実施し、すべて正常に合格（Passed）することを確認しました。

| 検証カテゴリ       | 検証項目                                                           | 結果 | 備考 / 検証方式                      |
| :----------------- | :----------------------------------------------------------------- | :--: | :----------------------------------- |
| **設定管理**       | `Config` の既定値初期化の整合性                                    | 合格 | `test_config_default_values`         |
| **永続化往復**     | `Config` の JSON シリアライズ/デシリアライズ完全復元               | 合格 | `test_config_json_serialization...`  |
| **設定フォールバック** | 未知/部分的な JSON 設定からの安全なデフォルト値補完               | 合格 | `test_config_json_partial_fallback`  |
| **スナップ（左端）** | 画面左端 15px 以内での自動吸着補正および負座標の境界値             | 合格 | `test_apply_snap_left_edge`          |
| **スナップ（右端）** | 画面右端 15px 以内での自動吸着補正                                 | 合格 | `test_apply_snap_right_edge`         |
| **スナップ（上端）** | 画面上端 15px 以内での自動吸着補正                                 | 合格 | `test_apply_snap_top_edge`           |
| **スナップ（下端）** | 画面下端 15px 以内での自動吸着補正                                 | 合格 | `test_apply_snap_bottom_edge`        |
| **スナップ（非対象）** | 画面端 15px 超での非吸着（元の座標維持）                           | 合格 | `test_apply_snap_no_snap_when...`    |
| **スナップ（四隅）** | コーナー部分での X/Y 2 軸同時吸着補正                              | 合格 | `test_apply_snap_corner_snap`        |
| **UI幅（全表示）** | 全コンポーネント有効時の必要ウィンドウ幅計算（1210px）              | 合格 | `test_calculate_window_width_all...` |
| **UI幅（最小制限）** | 全コンポーネント無効時の最小ウィンドウ幅クランプ（200px）          | 合格 | `test_calculate_window_width_all...` |
| **UI幅（任意選択）** | 任意トグル時の正確な横幅動的計算                                   | 合格 | `test_calculate_window_width_cus...` |
| **履歴キュー管理** | スパークライン用データ配列の最大長（30件）維持と FIFO スライド     | 合格 | `test_push_history_capacity`         |
| **ログローテーション** | `sensors_debug.log` の最大 3 世代バックアップシフト (`.1`, `.2`)   | 合格 | `test_rotate_logs_succession`        |
| **通信量差分計算** | 累積値差分計算およびアンダーフロー時のゼロ安全保護 (`saturating`)  | 合格 | `test_saturating_diff_calculation`   |
| **静的解析**       | `cargo clippy --all-targets -- -D warnings` エラー・警告ゼロ       | 合格 | Clippy 警告完全解消                  |
| **書式準拠**       | `cargo fmt --check` フォーマット完全準拠                           | 合格 | rustfmt 準拠                         |
| **ドキュメント**   | `cargo doc --no-deps --document-private-items` ビルド警告ゼロ      | 合格 | 全型・全関数 RustDoc 整備            |
| **二重起動防止**   | 多重起動時に後続プロセスが即座に正常終了する挙動                   | 合格 | `common_lib` Named Mutex 連携        |
| **アンカー移動**   | Win32 `GetCursorPos` によるマウス同期のスムーズなドラッグ移動     | 合格 | 振動・ズレなし                       |

---

## 3. パフォーマンス計測
- **CPU負荷:** 0.1% 未満（必要なメトリクスのみの最小限リフレッシュ、およびプロセスの限定更新）
- **メモリ使用量:** 約 15 〜 20 MB（常駐時）
- **バイナリサイズ:** Release ビルド時 `opt-level = 'z'`, `lto = true`, `panic = 'abort'`, `strip = true` により超軽量フットプリントを実現。
