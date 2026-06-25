# MiSysMon (Mini System Monitor - Rust Edition)

📖 [English Version](./README.md)

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)

低リソース環境（スペックの限られたWindows PC）に最適化された、超軽量・極細のデスクトップシステムモニター。

| CPU | MEM | NET | DISK | IO | CLOCK |
| :---: | :---: | :---: | :---: | :---: | :---: |
| 5.1% (42°C) | 12.3% | 1.2M^ / 0.8Mv | 123G/512G | 0.5MR / 0.1MW | 2026/04/21(火) 12:00:00 |

## 🌟 特徴

- **超軽量動作:** Pure Rust + `egui` (Immediate mode GUI) により、WebViewを使用するフレームワークよりも圧倒的に低いメモリ消費とCPU負荷。
- **極細・省スペース:** 1100x32px の横長バー形式。デスクトップの端に配置しても作業の邪魔になりません。
- **常時最前面 / 背景透過:** 他のウィンドウに隠れることなく、システム情報を常に把握。
- **位置記憶機能:** 終了時の位置を自動保存し、次回の起動時に同じ場所で開きます。
- **情報の透明性:** コンソール窓（DOS窓）を表示せず、スマートなGUIアプリとして動作。
- **二重起動防止機能:** Windows名前付きミューテックスを使用し、既にアプリケーションが起動している場合は即座に終了するため、リソースの浪費やUIの重複表示を防ぎます。

## 📊 監視項目

- **CPU:** 使用率 (%) および パッケージ温度 (°C) （※ Windowsのセキュリティ制限やハードウェア環境により取得できない場合は `--°C` と表示されます。詳細は `%APPDATA%\Mini System Monitor\sensors_debug.log` をご確認ください。）
- **Memory:** 使用率 (%) 
- **Network:** 実時間の上り (^) / 下り (v) 通信速度
- **Disk Space:** システムドライブの使用量と全容量
- **Disk I/O:** リアルタイムの読み込み (R) / 書き込み (W) 速度
- **Clock:** 日時・秒単位の時計

## 🛠️ セットアップとビルド

### プリリクエスト
- [Rust](https://www.rust-lang.org/ja/tools/install) (latest stable)

### ビルド手順
```powershell
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

### 設定データ（ウィンドウ位置など）の保存場所とリセット
ウィンドウの位置や設定情報は以下のファイルに保存されています。
`%APPDATA%\Mini System Monitor\data\app.ron`
*(フルパス: `C:\Users\<ユーザー名>\AppData\Roaming\Mini System Monitor\data\app.ron`)*

**位置がおかしくなった場合などのリセット手順:**
1. 本アプリを終了します（右端の「×」ボタン）。
2. 上記の `app.ron` ファイル（または `Mini System Monitor` フォルダごと）を削除します。
3. アプリを再起動すると、デフォルトの位置とサイズで起動し、設定ファイルが再生成されます。

## 📄 ライセンス
MIT License (詳細は [LICENSE](./LICENSE) を参照してください)
