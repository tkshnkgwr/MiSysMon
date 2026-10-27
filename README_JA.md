# MiSysMon (Mini System Monitor - Rust Edition)

📖 [English Version](./README.md)

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)
[![CI](https://github.com/tkshnkgwr/MiSysMon/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/MiSysMon/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/tkshnkgwr/MiSysMon?logo=github)](https://github.com/tkshnkgwr/MiSysMon/releases)

低リソース環境（スペックの限られたWindows PC）に最適化された、超軽量・極細のデスクトップシステムモニター。

|     CPU     |   CPU GRAPH   |  MEM  |   MEM GRAPH   |      NET      |   DISK    |      IO       |   VER   |          CLOCK           |
| :---------: | :-----------: | :---: | :-----------: | :-----------: | :-------: | :-----------: | :-----: | :----------------------: |
| 5.1% (42°C) | [ ▅▆▇▃▂  ]   | 12.3% | [ ▂▄▅▃   ]   | 1.2M^ / 0.8Mv | 123G/512G | 0.5MR / 0.1MW | v1.1.1  | 2026/09/01(Tue) 13:45:00 |

## 🌟 特徴

- **超軽量動作:** Pure Rust + `egui` (Immediate mode GUI) により、WebViewを使用するフレームワークよりも圧倒的に低いメモリ消費（約15〜20MB）とCPU負荷（<0.1%）。
- **極細・省スペース:** 横長スリムバー形式（高さ32px）。デスクトップの端に配置しても作業の邪魔になりません。
- **画面端オートスナップ（吸着）:** ウィンドウを画面端（15px以内）に近づけると、境界線にピタッと吸着。
- **絶対座標アンカー移動:** マウスポインタと完全に同期した、ズレや振動のないスムーズなドラッグ移動。
- **リアルタイムスパークライン:** CPUおよびメモリ使用率の直近30秒間の推移を高さ12pxの折れ線グラフでリアルタイム表示。
- **設定カスタマイズ画面:** グリップ（`=`）のダブルクリックまたは右クリックで設定画面を表示。透過度、更新間隔、スナップ、表示項目のトグルと動的リサイズが可能。
- **常時最前面 / 背景透過:** 他のウィンドウに隠れることなく、システム情報を常に把握。
- **位置記憶機能:** 終了時の位置を自動保存し、次回の起動時に同じ場所で開きます。
- **二重起動防止機能:** Windows名前付きミューテックスを使用し、既にアプリケーションが起動している場合は即座に終了。

## 📊 監視項目

- **CPU:** 使用率 (%)、パッケージ温度 (°C)、および直近30秒間のトレンドライン
- **Memory:** 使用率 (%) および直近30秒間のトレンドライン
- **Network:** 実時間の上り (^) / 下り (v) 通信速度
- **Disk Space:** システムドライブの使用量と全容量
- **Disk I/O:** リアルタイムの読み込み (R) / 書き込み (W) 速度
- **Version:** 現在のアプリケーションバージョン (`vX.Y.Z`)
- **Clock:** 日時・秒単位の時計

## 🛠️ セットアップとビルド

### プリリクエスト
- [Rust](https://www.rust-lang.org/ja/tools/install) (latest stable)

### ビルド手順
```powershell
# 共有ライブラリのクローン（同じ親ディレクトリに並べて配置）
git clone https://github.com/tkshnkgwr/common_lib.git

# リポジトリのクローン
git clone https://github.com/tkshnkgwr/MiSysMon.git
cd MiSysMon

# リリースビルド (最適化済み・単一バイナリ)
cargo build --release
```
実行ファイルは `target/release/mini-system-monitor.exe` に生成されます。

## 🎨 デザイン設計ガイドライン
本プロジェクトは、視認性と実用性を両立させるため、以下の設計原則に従っています。
- **Impact Style:** 数値の視認性を高める力強いタイポグラフィの採用。
- **Zero Distraction:** 点滅や過度なアニメーションを排除した安定したレイアウト。
- **Hardware Feel:** 物理的な計測機のような、精密で機能的なUI。

## 🔍 トラブルシューティング（温度が表示されない場合）
Windowsのセキュリティ制限（WMIアクセス許可）や、お使いのマザーボードのセンサー仕様によっては、CPU温度が正しく取得できない場合があります。
1. **管理者権限で実行する:** 実行ファイルを右クリックし「管理者として実行」をお試しください。
2. **ログの確認:** 設定フォルダに出力される `%APPDATA%\Mini System Monitor\sensors_debug.log` を確認してください。`Detected Sensors Count: 0` の場合は、Windows標準APIではハードウェアの温度情報にアクセスできません。

## ⚙️ 各種設定

### Windows 起動時の自動実行（常駐化）
本アプリを Windows 起動時に自動で起動させたい場合は、以下の手順でスタートアップに登録してください。
1. `target/release/mini-system-monitor.exe` のショートカットを作成します。
2. キーボードの `Win + R` キーを押し、「ファイル名を指定して実行」を開きます。
3. `shell:startup` と入力して Enter を押します（スタートアップフォルダが開きます）。
4. 開いたフォルダ内に、作成したショートカットを配置（コピーまたは移動）します。

### 設定データの保存場所とリセット
ウィンドウの位置や設定情報は以下のフォルダに保存されます。
`%APPDATA%\Mini System Monitor\`

**位置がおかしくなった場合などのリセット手順:**
1. 本アプリを終了します（右端の「×」ボタン）。
2. 上記の `Mini System Monitor` フォルダ内の設定ファイルを削除します。
3. アプリを再起動すると、デフォルトの位置とサイズで起動し、設定が再生成されます。

## 📚 内部ドキュメント
開発者や詳細な動作を確認したい管理者向けに、以下のガイドラインを用意しています。
- [機能仕様書](./docs/ja/SPEC.md): 製品の機能一覧と技術スタックの定義。
- [システム設計・アーキテクチャ](./docs/ja/ARCHITECTURE.md): 内部モジュール設計、データフロー、ディレクトリ構造の意図。
- [システム構成図 (Diagram)](./docs/ja/DIAGRAM.md): システム構成およびデータフローのMermaid図。
- [テスト方針・実行ガイド](./docs/ja/TESTING.md): ユニットテスト方針とモジュール別検証項目。
- [テスト結果報告書](./docs/ja/TEST_REPORT.md): 動作検証結果とパフォーマンス記録。
- [マルチリポジトリ開発ガイド](./docs/ja/DEVELOPMENT.md): `common_lib` と並行したローカル開発環境の構築手順。
- [リリース手順書](./docs/ja/RELEASE.md): バージョン更新およびリリース作業の手順マニュアル。
- [リリース・バージョン管理フロー手順書](./docs/ja/RELEASE_FLOW.md): GitHub Actionsによる自動リリースとトラブルシューティング。
- [貢献ガイドライン](./docs/ja/CONTRIBUTING.md): バグ報告、機能提案、プルリクエスト提出時のガイドライン。
- [セキュリティポリシー](./docs/ja/SECURITY.md): セキュリティ方針、サポート対象バージョン、脆弱性報告手順。
- [AI指示書（コーディング規約）](./docs/ja/INSTRUCTIONS.md): AIエージェント（大賢者）がコード変更時に遵守すべき規約。
- [開発変遷・パフォーマンス記録 (Footprints)](./docs/ja/FOOTPRINTS.md): 開発マイルストーンと動作フットプリントの変遷。
- [変更履歴 (Changelog)](./docs/ja/CHANGELOG.md): バージョンごとの詳細な変更履歴。
- [開発タスクリスト (Todo)](./docs/ja/TODO.md): 実装タスクと今後のロードマップ。
- [プロジェクトテンプレートガイド](./docs/ja/PROJECT_TEMPLATE_GUIDE.md): 新規Rustデスクトップアプリ開発の黄金ひな形。

## 📄 ライセンス

本プロジェクトは MIT License のもとで公開されています - Copyright (c) 2026 tkshnkgwr。詳細は [LICENSE](./LICENSE) をご覧ください。
