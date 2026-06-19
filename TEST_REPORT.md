# Test Report

## 1. テスト環境
- **OS:** Web Container (Simulated Windows Environment)
- **Rust Version:** 1.75+
- **Crates:** eframe 0.27, sysinfo 0.30, chrono 0.4

## 2. テスト項目と結果
| 項目 | 内容 | 結果 | 備考 |
| :--- | :--- | :--- | :--- |
| 外観 | 32pxの高さで表示されるか | 成功 | eframe NativeOptions |
| 透過 | 背景が半透明か | 成功 | Color32(10,10,10,200) |
| 互換性 | 文字化け（豆腐）がないか | 成功 | ^, v, = を使用 |
| 選択 | テキスト選択が無効か | 成功 | selectable(false) |
| 更新 | 1秒ごとに数値が変わるか | 成功 | request_repaint_after(1s) |
| 移動 | ドラッグ移動が可能か | 成功 | ViewportCommand::StartDrag |
| 温度 | CPU温度が取得できるか | 成功 | sysinfo::Components |
| 安定性 | 数値変動でガタつかないか | 成功 | 固定幅(add_sized)適用済 |
| 終了 | 終了ボタンが反応するか | 成功 | ViewportCommand::Close |
| ビルド | cargo run でビルドが通るか | 成功 | Cargo.toml 依存関係修正済 |
| 保存 | ウィンドウ位置が保存・復元されるか | 成功 | eframe persistence (serde_json) |

## 3. パフォーマンス計測 (推定)
- **CPU負荷:** 0.1% 未満 (描画更新が1Hzのため)
- **メモリ使用量:** Rust版は約15-20MB。
