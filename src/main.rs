#![windows_subsystem = "windows"]

//! # ミニシステムモニター (mini-system-monitor)
//!
//! Windows環境向けの軽量なシステムモニターアプリケーション。
//! egui および eframe を使用したデスクトップ常駐型の UI を提供し、
//! CPU使用率・温度、メモリ使用率、ネットワークトラフィック、ディスク使用量とIO、
//! 現在の時刻などのシステムメトリクスをリアルタイムで表示します。

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use sysinfo::{Components, Disks, Networks, System};

// UPDATE 2026-04-16: ウィンドウ位置の保存機能の追加と、表示切替機能の削除。

#[cfg(target_os = "windows")]
#[allow(clippy::upper_case_acronyms)]
mod win32 {
    use std::ffi::c_void;

    // HMONITOR はハンドル（ポインタ）型
    type HMONITOR = *mut c_void;

    #[repr(C)]
    pub struct POINT {
        pub x: i32,
        pub y: i32,
    }

    pub const MONITOR_DEFAULTTONEAREST: u32 = 2;

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    pub struct RECT {
        pub left: i32,
        pub top: i32,
        pub right: i32,
        pub bottom: i32,
    }

    #[repr(C)]
    pub struct MONITORINFO {
        pub cb_size: u32,
        pub rc_monitor: RECT,
        pub rc_work: RECT,
        pub dw_flags: u32,
    }

    extern "system" {
        fn MonitorFromPoint(pt: POINT, dw_flags: u32) -> HMONITOR;
        fn GetMonitorInfoW(h_monitor: HMONITOR, lpmi: *mut MONITORINFO) -> i32;
    }

    /// 指定したスクリーン座標 (x, y) が現在接続されているいずれかのモニターの表示領域内にあるかチェックします。
    pub fn is_position_on_any_monitor(x: i32, y: i32) -> bool {
        unsafe {
            let pt = POINT { x, y };
            let h_monitor = MonitorFromPoint(pt, 0); // MONITOR_DEFAULTTONULL
            !h_monitor.is_null()
        }
    }

    /// 指定した座標 (x, y) に対応するモニターの作業領域（タスクバー等を除く領域）を取得します。
    pub fn get_monitor_work_area(x: i32, y: i32) -> Option<RECT> {
        unsafe {
            let pt = POINT { x, y };
            let h_monitor = MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST);
            if h_monitor.is_null() {
                return None;
            }
            let mut info = MONITORINFO {
                cb_size: std::mem::size_of::<MONITORINFO>() as u32,
                rc_monitor: RECT::default(),
                rc_work: RECT::default(),
                dw_flags: 0,
            };
            if GetMonitorInfoW(h_monitor, &mut info) != 0 {
                Some(info.rc_work)
            } else {
                None
            }
        }
    }
}

/// アプリケーションの設定情報を保持する構造体。
///
/// ウィンドウの位置など、次回起動時に復元したい情報をシリアライズするために使用されます。
#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
struct Config {
    /// 終了時のウィンドウの左上座標（スクリーン座標系）。
    pos: Option<egui::Pos2>,
    /// メトリクスの更新間隔（秒）。デフォルト: 1.0秒。
    update_interval_secs: f32,
    /// メインウィンドウの背景不透明度 (0.3 〜 1.0)。デフォルト: 0.78。
    opacity: f32,
    /// ウィンドウの吸着（オートスナップ）を有効化するかどうか。デフォルト: true。
    enable_snap: bool,
    /// 各項目の表示/非表示フラグ。
    show_cpu: bool,
    show_mem: bool,
    show_net: bool,
    show_disk: bool,
    show_io: bool,
    show_version: bool,
    show_clock: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pos: None,
            update_interval_secs: 1.0,
            opacity: 0.78,
            enable_snap: true,
            show_cpu: true,
            show_mem: true,
            show_net: true,
            show_disk: true,
            show_io: true,
            show_version: true,
            show_clock: true,
        }
    }
}

/// 指定されたディレクトリ内の指定されたログファイルをローテーションします。
///
/// 最大 `max_backups` 個の過去ログファイルを保持します。
fn rotate_logs(dir: &std::path::Path, base_name: &str, max_backups: usize) {
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
struct SystemMonitor {
    /// システム全体の情報を取得・管理するための `sysinfo::System` インスタンス。
    sys: System,
    /// ネットワークインターフェースの情報を取得するための `sysinfo::Networks` インスタンス。
    networks: Networks,
    /// ディスクの情報を取得するための `sysinfo::Disks` インスタンス。
    disks: Disks,
    /// 温度センサーなどのコンポーネント情報を取得するための `sysinfo::Components` インスタンス。
    components: Components,
    /// 前回メトリクスを更新した時刻。1秒ごとの更新制御に使用します。
    last_update: Instant,
    /// 現在のCPU使用率 (0.0%〜100.0%)。
    cpu_usage: f32,
    /// 現在のCPU温度 (℃)。
    cpu_temp: f32,
    /// 現在のメモリ使用率 (0.0%〜100.0%)。
    mem_usage: f64,
    /// 前回の更新からの送信バイト数 (アップロード速度計算用)。
    net_up: u64,
    /// 前回の更新からの受信バイト数 (ダウンロード速度計算用)。
    net_down: u64,
    /// ディスク読み込み速度 (バイト/秒)。
    disk_read: u64,
    /// ディスク書き込み速度 (バイト/秒)。
    disk_write: u64,
    /// 前回の測定時の累積送信バイト数。
    prev_net_up: u64,
    /// 前回の測定時の累積受信バイト数。
    prev_net_down: u64,
    /// システムのディスク使用量合計 (GB)。
    disk_used: u64,
    /// システムの全ディスクの容量合計 (GB)。
    disk_total: u64,
    /// 保存・復元されるアプリケーション設定。
    config: Config,
    /// CPU使用率の履歴データ。
    cpu_history: Vec<f32>,
    /// メモリ使用率の履歴データ。
    mem_history: Vec<f32>,
    /// 設定画面が開いているかどうか。
    settings_open: bool,
}

impl SystemMonitor {
    /// 新しい `SystemMonitor` インスタンスを生成します。
    ///
    /// eframe の初期化コンテキストを受け取り、保存されたウィンドウ位置設定を読み込みます。
    /// また、検出された温度センサー情報をデバッグ用にファイル (`sensors_debug.log`) に出力します。
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut sys = System::new();
        sys.refresh_cpu_all();
        sys.refresh_memory();

        let components = Components::new_with_refreshed_list();

        // ログ出力先ディレクトリの決定
        let log_dir = if let Ok(appdata) = std::env::var("APPDATA") {
            std::path::PathBuf::from(appdata).join("Mini System Monitor")
        } else {
            std::path::PathBuf::from(".")
        };
        let _ = std::fs::create_dir_all(&log_dir);

        // ログファイルの簡易ローテーション実行（最大3バックアップ）
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
        // 初回の受信/送信バイト数を蓄積して prev_net_up / prev_net_down の初期値とする
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
        }
    }

    /// 各種システムメトリクスを更新します。
    ///
    /// CPU使用率・温度、メモリ使用率、ネットワークトラフィックの差分、
    /// ディスクIO速度およびディスク使用量を再計測して構造体のフィールドに格納します。
    fn update_stats(&mut self) {
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

        // CPU使用率
        self.cpu_usage = self.sys.global_cpu_usage();

        // CPU温度取得
        // 優先順位1: 明示的なCPU/温度に関連するラベルを持つセンサー
        let mut temp = self
            .components
            .iter()
            .find(|c| {
                let label = c.label().to_uppercase();
                label.contains("CPU") || 
                label.contains("CORE") || 
                label.contains("PACKAGE") ||
                label.contains("TCTL") || // AMD
                label.contains("TDIE") || // AMD
                label.contains("THM") ||  // Thermal
                label.contains("TEMP") // Generic
            })
            .and_then(|c| c.temperature())
            .unwrap_or(0.0);

        // 優先順位2: 上記で見つからない（または0度）場合、全センサーの中で最高温度を採用（最も負荷が高い部位＝CPUの可能性大）
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

        // ネットワーク速度 (差分計算)
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

        // ディスクIO速度 (プロセスごとの合計)
        let mut total_read = 0;
        let mut total_write = 0;
        for process in self.sys.processes().values() {
            let usage = process.disk_usage();
            total_read += usage.read_bytes;
            total_write += usage.written_bytes;
        }
        self.disk_read = total_read;
        self.disk_write = total_write;

        // ディスク使用量
        let mut d_total = 0;
        let mut d_available = 0;
        for disk in &self.disks {
            d_total += disk.total_space();
            d_available += disk.available_space();
        }
        self.disk_used = (d_total - d_available) / 1024 / 1024 / 1024; // GB
        self.disk_total = d_total / 1024 / 1024 / 1024; // GB

        // 履歴の更新
        self.cpu_history.push(self.cpu_usage);
        if self.cpu_history.len() > 30 {
            self.cpu_history.remove(0);
        }
        self.mem_history.push(self.mem_usage as f32);
        if self.mem_history.len() > 30 {
            self.mem_history.remove(0);
        }
    }
}

#[cfg(target_os = "windows")]
fn apply_snap(pos: egui::Pos2, size: egui::Vec2, work_area: win32::RECT) -> egui::Pos2 {
    let mut x = pos.x;
    let mut y = pos.y;
    let width = size.x;
    let height = size.y;

    let threshold = 15.0; // 吸着のしきい値（ピクセル）

    // 左右の吸着
    if (x - work_area.left as f32).abs() < threshold {
        x = work_area.left as f32;
    } else if ((x + width) - work_area.right as f32).abs() < threshold {
        x = work_area.right as f32 - width;
    }

    // 上下の吸着
    if (y - work_area.top as f32).abs() < threshold {
        y = work_area.top as f32;
    } else if ((y + height) - work_area.bottom as f32).abs() < threshold {
        y = work_area.bottom as f32 - height;
    }

    egui::Pos2::new(x, y)
}

fn draw_trend_line(ui: &mut egui::Ui, history: &[f32], max_val: f32) {
    let desired_size = egui::vec2(40.0, 12.0);
    let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();

        // 背景枠
        painter.rect_filled(
            rect,
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 10),
        );

        if history.len() > 1 {
            let pts: Vec<egui::Pos2> = history
                .iter()
                .enumerate()
                .map(|(i, &val)| {
                    let x = rect.min.x + (i as f32 / (history.len() - 1) as f32) * rect.width();
                    let ratio = (val / max_val).clamp(0.0, 1.0);
                    let y = rect.max.y - ratio * rect.height();
                    egui::Pos2::new(x, y)
                })
                .collect();

            for window in pts.windows(2) {
                painter.line_segment(
                    [window[0], window[1]],
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 255, 255)), // シアン
                );
            }
        }
    }
}

impl SystemMonitor {
    /// 現在の表示項目に基づいて必要なウィンドウ幅を動的に計算します。
    fn calculate_width(&self) -> f32 {
        let mut width: f32 = 60.0; // 左右マージン + グリップ + 終了ボタンなど

        if self.config.show_cpu {
            width += 155.0;
        }
        if self.config.show_mem {
            width += 115.0;
        }
        if self.config.show_net {
            width += 155.0;
        }
        if self.config.show_disk {
            width += 125.0;
        }
        if self.config.show_io {
            width += 155.0;
        }
        if self.config.show_version {
            width += 60.0;
        }
        if self.config.show_clock {
            width += 195.0;
        }

        width.max(200.0) // 最小幅を制限
    }
}

impl eframe::App for SystemMonitor {
    /// アプリケーション終了時に設定情報を永続化ストレージに保存します。
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Ok(json) = serde_json::to_string(&self.config) {
            storage.set_string(eframe::APP_KEY, json);
        }
    }

    /// UIの描画および更新ロジックを定義します。
    ///
    /// メトリクス情報の更新を行うほか、ウィンドウのドラッグ移動処理、
    /// 各項目のレイアウト、終了ボタンの制御、時計の表示を行います。
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 現在のウィンドウ位置を監視して保存用に更新 (outer_rect.min -> position)
        ui.input(|i| {
            if let Some(rect) = i.viewport().outer_rect {
                self.config.pos = Some(rect.min);
            }
        });

        // 設定された更新間隔ごとにデータを更新
        let interval = Duration::from_secs_f32(self.config.update_interval_secs);
        if self.last_update.elapsed() >= interval {
            self.update_stats();
            self.last_update = Instant::now();
        }

        // ウィンドウ全体のスタイル設定
        let alpha = (self.config.opacity * 255.0) as u8;
        let panel_frame = egui::Frame::new()
            .fill(egui::Color32::from_rgba_unmultiplied(10, 10, 10, alpha)) // 設定値による背景透過
            .inner_margin(egui::Margin::symmetric(10, 5));

        // 右クリックを検知して設定画面をトグル
        ui.input(|i| {
            if i.pointer.secondary_clicked() {
                self.settings_open = !self.settings_open;
            }
        });

        let current_width = self.calculate_width();

        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ui, |ui| {
                // テキスト選択を無効化
                ui.style_mut().interaction.selectable_labels = false;

                ui.horizontal(|ui| {
                    // 自前ドラッグハンドルの実装
                    let drag_label = ui.add(
                        egui::Label::new(
                            egui::RichText::new("=")
                                .size(14.0)
                                .color(egui::Color32::from_gray(100)),
                        )
                        .sense(egui::Sense::drag()),
                    );

                    // ダブルクリックで設定画面をトグル
                    if drag_label.double_clicked() {
                        self.settings_open = !self.settings_open;
                    }

                    if drag_label.dragged() {
                        let delta = drag_label.drag_delta();
                        if delta != egui::Vec2::ZERO {
                            if let Some(pos) = self.config.pos {
                                #[allow(unused_mut)]
                                let mut new_pos = pos + delta;

                                #[cfg(target_os = "windows")]
                                if self.config.enable_snap {
                                    if let Some(work_area) = win32::get_monitor_work_area(
                                        new_pos.x as i32,
                                        new_pos.y as i32,
                                    ) {
                                        new_pos = apply_snap(
                                            new_pos,
                                            egui::vec2(current_width, 32.0),
                                            work_area,
                                        );
                                    }
                                }

                                self.config.pos = Some(new_pos);
                                ui.ctx()
                                    .send_viewport_cmd(egui::ViewportCommand::OuterPosition(
                                        new_pos,
                                    ));
                            }
                        }
                    }

                    ui.add_space(10.0);
                    ui.spacing_mut().item_spacing.x = 4.0;

                    let label_style = |ui: &mut egui::Ui, name: &str, val: String, width: f32| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 4.0;
                            ui.label(
                                egui::RichText::new(name)
                                    .size(10.0)
                                    .color(egui::Color32::from_gray(120)),
                            );
                            ui.add_sized(
                                [width, 12.0],
                                egui::Label::new(
                                    egui::RichText::new(val)
                                        .strong()
                                        .size(12.0)
                                        .color(egui::Color32::WHITE),
                                )
                                .selectable(false),
                            );
                        });
                    };

                    // 各項目の表示トグルとトレンドラインの描画
                    if self.config.show_cpu {
                        let temp_str = if self.cpu_temp > 0.0 {
                            format!("{:>2.0}°C", self.cpu_temp)
                        } else {
                            "--°C".to_string()
                        };
                        label_style(
                            ui,
                            "CPU",
                            format!("{:>5.1}% ({})", self.cpu_usage, temp_str),
                            100.0,
                        );
                        draw_trend_line(ui, &self.cpu_history, 100.0);
                        ui.add_space(15.0);
                    }

                    if self.config.show_mem {
                        label_style(ui, "MEM", format!("{:>5.1}%", self.mem_usage), 60.0);
                        draw_trend_line(ui, &self.mem_history, 100.0);
                        ui.add_space(15.0);
                    }

                    if self.config.show_net {
                        label_style(
                            ui,
                            "NET",
                            format!(
                                "{:>6}^ / {:>6}v",
                                common_lib::format_bytes(self.net_up),
                                common_lib::format_bytes(self.net_down)
                            ),
                            140.0,
                        );
                        ui.add_space(15.0);
                    }

                    if self.config.show_disk {
                        label_style(
                            ui,
                            "DISK",
                            format!("{:>4}GB/{:>4}GB", self.disk_used, self.disk_total),
                            110.0,
                        );
                        ui.add_space(15.0);
                    }

                    if self.config.show_io {
                        label_style(
                            ui,
                            "IO",
                            format!(
                                "{:>6}R / {:>6}W",
                                common_lib::format_bytes(self.disk_read),
                                common_lib::format_bytes(self.disk_write)
                            ),
                            140.0,
                        );
                        ui.add_space(15.0);
                    }

                    if self.config.show_version {
                        label_style(ui, "VER", format!("v{}", env!("CARGO_PKG_VERSION")), 45.0);
                        ui.add_space(15.0);
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("×").clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }

                        if self.config.show_clock {
                            let now = chrono::Local::now();
                            let time_str = now.format("%Y/%m/%d(%a) %H:%M:%S").to_string();

                            // 時計表示 (点滅なし、固定幅)
                            ui.add_sized(
                                [180.0, 12.0],
                                egui::Label::new(
                                    egui::RichText::new(time_str).strong().size(12.0).color(
                                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 230),
                                    ),
                                ),
                            );
                            ui.add_space(10.0);
                        }
                    });
                });
            });

        // 設定画面を描画
        let settings_open = self.settings_open;
        if settings_open {
            let mut open = settings_open;
            let mut config_changed = false;

            ui.ctx().show_viewport_immediate(
                egui::ViewportId::from_hash_of("settings_window"),
                egui::ViewportBuilder::default()
                    .with_title("MiSysMon Settings")
                    .with_inner_size([300.0, 430.0])
                    .with_resizable(false)
                    .with_decorations(true)
                    .with_always_on_top(),
                |ctx, class| {
                    if class != egui::ViewportClass::Immediate {
                        return;
                    }

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("MiSysMon Settings");
                        ui.separator();

                        ui.group(|ui| {
                            ui.label("Display Options");
                            if ui.checkbox(&mut self.config.show_cpu, "Show CPU").changed() {
                                config_changed = true;
                            }
                            if ui
                                .checkbox(&mut self.config.show_mem, "Show Memory")
                                .changed()
                            {
                                config_changed = true;
                            }
                            if ui
                                .checkbox(&mut self.config.show_net, "Show Network")
                                .changed()
                            {
                                config_changed = true;
                            }
                            if ui
                                .checkbox(&mut self.config.show_disk, "Show Disk")
                                .changed()
                            {
                                config_changed = true;
                            }
                            if ui
                                .checkbox(&mut self.config.show_io, "Show Disk IO")
                                .changed()
                            {
                                config_changed = true;
                            }
                            if ui
                                .checkbox(&mut self.config.show_version, "Show Version")
                                .changed()
                            {
                                config_changed = true;
                            }
                            if ui
                                .checkbox(&mut self.config.show_clock, "Show Clock")
                                .changed()
                            {
                                config_changed = true;
                            }
                        });

                        ui.add_space(10.0);

                        ui.group(|ui| {
                            ui.label("Window Options");
                            if ui
                                .checkbox(
                                    &mut self.config.enable_snap,
                                    "Enable Window Snapping (15px)",
                                )
                                .changed()
                            {
                                config_changed = true;
                            }

                            ui.add_space(5.0);

                            ui.label("Opacity");
                            let prev_opacity = self.config.opacity;
                            ui.add(egui::Slider::new(&mut self.config.opacity, 0.3..=1.0).text(""));
                            if self.config.opacity != prev_opacity {
                                config_changed = true;
                            }

                            ui.add_space(5.0);

                            ui.label("Update Interval");
                            let prev_interval = self.config.update_interval_secs;
                            ui.add(
                                egui::Slider::new(&mut self.config.update_interval_secs, 0.5..=5.0)
                                    .step_by(0.5)
                                    .suffix("s"),
                            );
                            if self.config.update_interval_secs != prev_interval {
                                config_changed = true;
                            }
                        });

                        ui.add_space(15.0);

                        ui.horizontal(|ui| {
                            if ui.button("Close").clicked() {
                                open = false;
                            }
                        });
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        open = false;
                    }
                },
            );

            self.settings_open = open;

            if config_changed {
                let new_width = self.calculate_width();
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                        new_width, 32.0,
                    )));
            }
        }

        // 動的サイズを強制
        ui.ctx()
            .send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                current_width,
                32.0,
            )));

        ui.ctx().request_repaint_after(Duration::from_secs_f32(
            self.config.update_interval_secs.min(1.0),
        ));
    }
}

/// アプリケーションのエントリーポイント。
///
/// 単一インスタンスの起動制御、ウィンドウの初期オプション（サイズ、リサイズ禁止、枠なし、常に最前面など）の設定、
/// および `eframe::run_native` による UI イベントループの起動を行います。
fn main() -> eframe::Result<()> {
    let _guard = match common_lib::desktop::acquire_single_instance(
        "Local\\mini-system-monitor-single-instance-key",
    ) {
        Some(guard) => guard,
        None => return Ok(()),
    };

    // 初期設定の読み込み（位置復元のため）
    let storage_name = eframe::APP_KEY;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 32.0])
            .with_resizable(false) // リサイズ禁止
            .with_decorations(false) // タイトルバー非表示
            .with_transparent(true) // 背景透過
            .with_always_on_top(), // 最前面
        ..Default::default()
    };

    eframe::run_native(
        "Mini System Monitor",
        options,
        Box::new(|cc| {
            // 保存された位置がある場合は適用 (eframe::get_value -> cc.storage.get_string)
            let config: Config = cc
                .storage
                .and_then(|s| s.get_string(storage_name))
                .and_then(|json| serde_json::from_str(&json).ok())
                .unwrap_or_default();

            let monitor = SystemMonitor::new(cc);
            let initial_width = monitor.calculate_width();

            if let Some(pos) = config.pos {
                // ウィンドウ位置が有効なモニターの表示範囲内にあるか検証
                #[cfg(target_os = "windows")]
                let is_valid = win32::is_position_on_any_monitor(pos.x as i32, pos.y as i32);
                #[cfg(not(target_os = "windows"))]
                let is_valid = true;

                if is_valid {
                    cc.egui_ctx
                        .send_viewport_cmd(egui::ViewportCommand::OuterPosition(pos));
                }
            }

            // 起動時に動的サイズを設定
            cc.egui_ctx
                .send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                    initial_width,
                    32.0,
                )));

            Ok(Box::new(monitor))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.pos.is_none());
    }
}
