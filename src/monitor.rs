//! # システムメトリクス監視・管理モジュール (`monitor`)
//!
//! `sysinfo` を用いた CPU、メモリ、ネットワーク、ディスク、および温度センサーの
//! リアルタイムメトリクス取得・状態管理機能を提供します。

use crate::config::Config;
use std::time::Instant;
use sysinfo::{Components, Disks, Networks, System};

/// 指定されたディレクトリ内の指定されたログファイルをローテーションします。
///
/// 最大 `max_backups` 個の過去ログファイルを保持します。
/// 例: `app.log` -> `app.log.1` -> `app.log.2` -> `app.log.3` (古いものは削除)
///
/// # 引数
/// - `dir`: ログファイルが存在するディレクトリのパス
/// - `base_name`: 対象となるログファイルのファイル名（例: `"sensors_debug.log"`）
/// - `max_backups`: 保持する最大バックアップ世代数
pub fn rotate_logs(dir: &std::path::Path, base_name: &str, max_backups: usize) {
    let oldest_path = dir.join(format!("{}.{}", base_name, max_backups));
    if oldest_path.exists() {
        let _ = std::fs::remove_file(oldest_path);
    }

    for i in (1..max_backups).rev() {
        let src = dir.join(format!("{}.{}", base_name, i));
        let dest = dir.join(format!("{}.{}", base_name, i + 1));
        if src.exists() {
            let _ = std::fs::rename(src, dest);
        }
    }

    let current_log = dir.join(base_name);
    let first_backup = dir.join(format!("{}.1", base_name));
    if current_log.exists() {
        let _ = std::fs::rename(current_log, first_backup);
    }
}

/// システムモニターの本体となるアプリケーション状態管理構造体。
///
/// 各種システムメトリクスの現在値や、前回の更新時刻、
/// `sysinfo` クレートのシステムハンドラなどを管理します。
pub struct SystemMonitor {
    /// システム全体の情報を取得・管理するための `sysinfo::System` インスタンス。
    pub sys: System,

    /// ネットワークインターフェースの情報を取得するための `sysinfo::Networks` インスタンス。
    pub networks: Networks,

    /// ディスクの情報を取得するための `sysinfo::Disks` インスタンス。
    pub disks: Disks,

    /// 温度センサーなどのコンポーネント情報を取得するための `sysinfo::Components` インスタンス。
    pub components: Components,

    /// 前回メトリクスを更新した時刻。設定されたインターバルごとの更新制御に使用します。
    pub last_update: Instant,

    /// 現在の CPU 使用率 (0.0% 〜 100.0%)。
    pub cpu_usage: f32,

    /// 現在の CPU 温度 (℃)。検出できない場合は `0.0`。
    pub cpu_temp: f32,

    /// 現在のメモリ使用率 (0.0% 〜 100.0%)。
    pub mem_usage: f64,

    /// 前回の更新からの送信バイト数 (アップロード速度 B/s)。
    pub net_up: u64,

    /// 前回の更新からの受信バイト数 (ダウンロード速度 B/s)。
    pub net_down: u64,

    /// ディスク読み込み速度 (B/s)。
    pub disk_read: u64,

    /// ディスク書き込み速度 (B/s)。
    pub disk_write: u64,

    /// 前回の測定時の累積送信バイト数。
    pub prev_net_up: u64,

    /// 前回の測定時の累積受信バイト数。
    pub prev_net_down: u64,

    /// システムのディスク使用量合計 (GB)。
    pub disk_used: u64,

    /// システムの全ディスクの容量合計 (GB)。
    pub disk_total: u64,

    /// 保存・復元されるアプリケーション設定。
    pub config: Config,

    /// CPU 使用率の直近履歴データ（スパークライン表示用、最大 30 点）。
    pub cpu_history: Vec<f32>,

    /// メモリ使用率の直近履歴データ（スパークライン表示用、最大 30 点）。
    pub mem_history: Vec<f32>,

    /// 設定サブウィンドウが開いているかどうかのフラグ。
    pub settings_open: bool,

    /// 最後に適用したメインウィンドウの横幅（px）。
    pub last_width: f32,

    /// ドラッグ移動開始時のウィンドウ座標。
    pub drag_start_win_pos: Option<eframe::egui::Pos2>,

    /// ドラッグ移動開始時の画面絶対マウス座標。
    pub drag_start_mouse_pos: Option<eframe::egui::Pos2>,
}

impl SystemMonitor {
    /// 新しい `SystemMonitor` インスタンスを生成します。
    ///
    /// eframe の初期化コンテキストを受け取り、保存された設定を復元します。
    /// また、検出された温度センサー情報をデバッグログ (`sensors_debug.log`) に出力します。
    ///
    /// # 引数
    /// - `cc`: eframe の起動コンテキスト (`CreationContext`)
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut sys = System::new();
        sys.refresh_cpu_all();
        sys.refresh_memory();

        let components = Components::new_with_refreshed_list();

        // ログ出力先ディレクトリの決定 (%APPDATA%/Mini System Monitor)
        let log_dir = if let Ok(appdata) = std::env::var("APPDATA") {
            std::path::PathBuf::from(appdata).join("Mini System Monitor")
        } else {
            std::path::PathBuf::from(".")
        };
        let _ = std::fs::create_dir_all(&log_dir);

        // ログファイルの世代ローテーション実行（最大3バックアップ）
        rotate_logs(&log_dir, "sensors_debug.log", 3);

        let log_path = log_dir.join("sensors_debug.log");
        if let Ok(mut file) = std::fs::File::create(log_path) {
            use std::io::Write;
            let _ = writeln!(file, "Detected Sensors Count: {}", components.len());
            if components.is_empty() {
                let _ = writeln!(
                    file,
                    "Note: If zero sensors are detected, you may need to run this application as Administrator to access CPU temperature sensors on Windows."
                );
            }
            for c in &components {
                let temp_str = c
                    .temperature()
                    .map_or("N/A".to_string(), |t| format!("{:.1}", t));
                let _ = writeln!(file, "Label: '{}', Temp: {}°C", c.label(), temp_str);
            }
        }

        // 保存された設定の読み込み
        let config: Config = cc
            .storage
            .and_then(|s| s.get_string(eframe::APP_KEY))
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default();

        let networks = Networks::new_with_refreshed_list();
        // 初回の受信/送信バイト数を蓄積して初期スパイクを防止
        let mut prev_net_up = 0;
        let mut prev_net_down = 0;
        for (_, data) in &networks {
            prev_net_up += data.transmitted();
            prev_net_down += data.received();
        }

        Self {
            sys,
            networks,
            disks: Disks::new_with_refreshed_list(),
            components,
            last_update: Instant::now(),
            cpu_usage: 0.0,
            cpu_temp: 0.0,
            mem_usage: 0.0,
            net_up: 0,
            net_down: 0,
            disk_read: 0,
            disk_write: 0,
            prev_net_up,
            prev_net_down,
            disk_used: 0,
            disk_total: 0,
            config,
            cpu_history: Vec::new(),
            mem_history: Vec::new(),
            settings_open: false,
            last_width: 0.0,
            drag_start_win_pos: None,
            drag_start_mouse_pos: None,
        }
    }

    /// 各種システムメトリクス（CPU、メモリ、ネットワーク、ディスク、温度）を更新します。
    ///
    /// 直近の差分からネットワークおよびディスク I/O 速度を算出し、
    /// トレンドライン表示用の履歴キュー（最大 30 点）を更新します。
    pub fn update_stats(&mut self) {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        self.sys.refresh_processes_specifics(
            sysinfo::ProcessesToUpdate::All,
            true,
            sysinfo::ProcessRefreshKind::nothing().with_disk_usage(),
        );
        self.networks.refresh(true);
        self.disks.refresh(false);
        self.components.refresh(false);

        // CPU 使用率
        self.cpu_usage = self.sys.global_cpu_usage();

        // CPU 温度取得ロジック
        // 優先度 1: ラベルに CPU 関連キーワードが含まれるセンサー
        let mut temp = self
            .components
            .iter()
            .find(|c| {
                let label = c.label().to_uppercase();
                label.contains("CPU")
                    || label.contains("CORE")
                    || label.contains("PACKAGE")
                    || label.contains("TCTL")
                    || label.contains("TDIE")
                    || label.contains("THM")
                    || label.contains("TEMP")
            })
            .and_then(|c| c.temperature())
            .unwrap_or(0.0);

        // 優先度 2: 上記で見つからない場合、全センサーの最高温度を採用
        if temp <= 0.0 {
            temp = self
                .components
                .iter()
                .filter_map(|c| c.temperature())
                .fold(0.0, f32::max);
        }
        self.cpu_temp = temp;

        // メモリ使用率
        let total_mem = self.sys.total_memory() as f64;
        let used_mem = self.sys.used_memory() as f64;
        if total_mem > 0.0 {
            self.mem_usage = (used_mem / total_mem) * 100.0;
        }

        // ネットワーク通信速度 (直前の累積値との差分計算)
        let mut total_up = 0;
        let mut total_down = 0;
        for (_, data) in &self.networks {
            total_up += data.transmitted();
            total_down += data.received();
        }
        self.net_up = total_up.saturating_sub(self.prev_net_up);
        self.net_down = total_down.saturating_sub(self.prev_net_down);
        self.prev_net_up = total_up;
        self.prev_net_down = total_down;

        // ディスク I/O 速度 (全プロセスのディスク読み書き合計)
        let mut total_read = 0;
        let mut total_write = 0;
        for process in self.sys.processes().values() {
            let usage = process.disk_usage();
            total_read += usage.read_bytes;
            total_write += usage.written_bytes;
        }
        self.disk_read = total_read;
        self.disk_write = total_write;

        // ディスク使用量 (全ディスクの合算)
        let mut d_total = 0;
        let mut d_available = 0;
        for disk in &self.disks {
            d_total += disk.total_space();
            d_available += disk.available_space();
        }
        self.disk_used = (d_total - d_available) / 1024 / 1024 / 1024; // GB
        self.disk_total = d_total / 1024 / 1024 / 1024; // GB

        // 履歴キューの更新（最大30件保持）
        Self::push_history(&mut self.cpu_history, self.cpu_usage, 30);
        Self::push_history(&mut self.mem_history, self.mem_usage as f32, 30);
    }

    /// 履歴スライスに新しい値を追加し、最大保持件数を超えた場合は古いデータを削除します。
    pub fn push_history(history: &mut Vec<f32>, new_val: f32, max_len: usize) {
        history.push(new_val);
        if history.len() > max_len {
            history.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_history_capacity() {
        let mut history = Vec::new();

        // 30件追加
        for i in 1..=30 {
            SystemMonitor::push_history(&mut history, i as f32, 30);
        }
        assert_eq!(history.len(), 30);
        assert_eq!(history[0], 1.0);
        assert_eq!(history[29], 30.0);

        // 31件目を追加すると先頭 (1.0) が破棄され長さ 30 を維持
        SystemMonitor::push_history(&mut history, 31.0, 30);
        assert_eq!(history.len(), 30);
        assert_eq!(history[0], 2.0);
        assert_eq!(history[29], 31.0);
    }

    #[test]
    fn test_rotate_logs_succession() {
        let temp_dir = std::env::temp_dir().join(format!("misysmon_test_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);

        let log_name = "test.log";

        // 1回目のファイル作成
        let log_file = temp_dir.join(log_name);
        std::fs::write(&log_file, "First Log Content").expect("書き込み成功");

        // 1回目のローテーション -> test.log.1 にリネームされる
        rotate_logs(&temp_dir, log_name, 3);
        assert!(!log_file.exists());
        assert!(temp_dir.join("test.log.1").exists());

        // 2回目のファイル作成 & ローテーション
        std::fs::write(&log_file, "Second Log Content").expect("書き込み成功");
        rotate_logs(&temp_dir, log_name, 3);
        assert!(temp_dir.join("test.log.1").exists());
        assert!(temp_dir.join("test.log.2").exists());

        // クリーンアップ
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_saturating_diff_calculation() {
        let prev: u64 = 1000;
        let curr: u64 = 1500;
        let diff = curr.saturating_sub(prev);
        assert_eq!(diff, 500);

        // 逆転（再起動やカウンターリセット時）
        let curr_smaller: u64 = 500;
        let diff_underflow = curr_smaller.saturating_sub(prev);
        assert_eq!(diff_underflow, 0);
    }
}
