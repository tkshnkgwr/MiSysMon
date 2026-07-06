#![windows_subsystem = "windows"]

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use sysinfo::{Components, Disks, Networks, System};

// UPDATE 2026-04-16: ウィンドウ位置の保存機能の追加と、表示切替機能の削除。

#[derive(Serialize, Deserialize, Default)]
struct Config {
    pos: Option<egui::Pos2>,
}

struct SystemMonitor {
    sys: System,
    networks: Networks,
    disks: Disks,
    components: Components,
    last_update: Instant,
    cpu_usage: f32,
    cpu_temp: f32,
    mem_usage: f64,
    net_up: u64,
    net_down: u64,
    disk_read: u64,
    disk_write: u64,
    prev_net_up: u64,
    prev_net_down: u64,
    disk_used: u64,
    disk_total: u64,
    config: Config,
}

impl SystemMonitor {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut sys = System::new();
        sys.refresh_cpu_all();
        sys.refresh_memory();

        let components = Components::new_with_refreshed_list();

        // DEBUG: センサー情報をファイルに出力（診断用）
        let mut log_path = if let Ok(appdata) = std::env::var("APPDATA") {
            std::path::PathBuf::from(appdata).join("Mini System Monitor")
        } else {
            std::path::PathBuf::from(".")
        };
        let _ = std::fs::create_dir_all(&log_path);
        log_path.push("sensors_debug.log");

        if let Ok(mut file) = std::fs::File::create(log_path) {
            use std::io::Write;
            let _ = writeln!(file, "Detected Sensors Count: {}", components.len());
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

        Self {
            sys,
            networks: Networks::new_with_refreshed_list(),
            disks: Disks::new_with_refreshed_list(),
            components, // 既に作成済みのインスタンスを使用
            last_update: Instant::now(),
            cpu_usage: 0.0,
            cpu_temp: 0.0,
            mem_usage: 0.0,
            net_up: 0,
            net_down: 0,
            disk_read: 0,
            disk_write: 0,
            prev_net_up: 0,
            prev_net_down: 0,
            disk_used: 0,
            disk_total: 0,
            config,
        }
    }

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
    }

    fn format_bytes(bytes: u64) -> String {
        if bytes < 1024 {
            format!("{}B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1}K", bytes as f32 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.1}M", bytes as f32 / 1024.0 / 1024.0)
        } else {
            format!("{:.1}G", bytes as f32 / 1024.0 / 1024.0 / 1024.0)
        }
    }
}

impl eframe::App for SystemMonitor {
    // 終了時に設定を保存 (eframe::set_value -> storage.set_string)
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Ok(json) = serde_json::to_string(&self.config) {
            storage.set_string(eframe::APP_KEY, json);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 現在のウィンドウ位置を監視して保存用に更新 (outer_rect.min -> position)
        ui.input(|i| {
            if let Some(rect) = i.viewport().outer_rect {
                self.config.pos = Some(rect.min);
            }
        });

        // 1秒ごとにデータを更新
        if self.last_update.elapsed() >= Duration::from_secs(1) {
            self.update_stats();
            self.last_update = Instant::now();
        }

        // ウィンドウ全体のスタイル設定
        let panel_frame = egui::Frame::new()
            .fill(egui::Color32::from_rgba_unmultiplied(10, 10, 10, 200)) // 背景透過
            .inner_margin(egui::Margin::symmetric(10, 5));

        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ui, |ui| {
                // テキスト選択を無効化
                ui.style_mut().interaction.selectable_labels = false;

                ui.horizontal(|ui| {
                    // ドラッグ移動の有効化 (文字化け対策: ☷ -> =)
                    let drag_label = ui.label(
                        egui::RichText::new("=")
                            .size(14.0)
                            .color(egui::Color32::from_gray(100)),
                    );
                    if drag_label.interact(egui::Sense::drag()).dragged() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
                    }

                    ui.spacing_mut().item_spacing.x = 15.0;

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

                    // 各項目の幅を固定してガタつきを防止
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
                    label_style(ui, "MEM", format!("{:>5.1}%", self.mem_usage), 60.0);
                    // 文字化け対策: ↑ -> ^, ↓ -> v
                    label_style(
                        ui,
                        "NET",
                        format!(
                            "{:>6}^ / {:>6}v",
                            Self::format_bytes(self.net_up),
                            Self::format_bytes(self.net_down)
                        ),
                        140.0,
                    );
                    label_style(
                        ui,
                        "DISK",
                        format!("{:>4}GB/{:>4}GB", self.disk_used, self.disk_total),
                        110.0,
                    );
                    label_style(
                        ui,
                        "IO",
                        format!(
                            "{:>6}R / {:>6}W",
                            Self::format_bytes(self.disk_read),
                            Self::format_bytes(self.disk_write)
                        ),
                        140.0,
                    );
                    label_style(ui, "VER", format!("v{}", env!("CARGO_PKG_VERSION")), 45.0);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("×").clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }

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
                    });
                });
            });

        ui.ctx().request_repaint_after(Duration::from_secs(1));
    }
}

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
            .with_min_inner_size([1100.0, 32.0]) // 最小サイズを固定
            .with_max_inner_size([1100.0, 32.0]) // 最大サイズを固定
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

            if let Some(pos) = config.pos {
                cc.egui_ctx
                    .send_viewport_cmd(egui::ViewportCommand::OuterPosition(pos));
                // サイズが勝手に変わらないよう、起動時にもサイズを強制
                cc.egui_ctx
                    .send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(1100.0, 32.0)));
            }

            Ok(Box::new(SystemMonitor::new(cc)))
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

    #[test]
    fn test_format_bytes() {
        assert_eq!(SystemMonitor::format_bytes(0), "0B");
        assert_eq!(SystemMonitor::format_bytes(512), "512B");
        assert_eq!(SystemMonitor::format_bytes(1023), "1023B");
        assert_eq!(SystemMonitor::format_bytes(1024), "1.0K");
        assert_eq!(SystemMonitor::format_bytes(1536), "1.5K");
        assert_eq!(SystemMonitor::format_bytes(1048576), "1.0M");
        assert_eq!(SystemMonitor::format_bytes(1572864), "1.5M");
        assert_eq!(SystemMonitor::format_bytes(1073741824), "1.0G");
        assert_eq!(SystemMonitor::format_bytes(2147483648), "2.0G");
    }
}
