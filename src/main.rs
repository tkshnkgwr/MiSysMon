#![windows_subsystem = "windows"]

//! # ミニシステムモニター (mini-system-monitor / MiSysMon)
//!
//! Windows環境向けの極めて軽量なデスクトップ常駐型システムモニターアプリケーションです。
//! `egui` および `eframe` を使用したスリムな横長バー UI を提供し、
//! CPU使用率・温度、メモリ使用率、ネットワークトラフィック、ディスク使用量とIO、
//! 現在の時刻などの各種システムメトリクスをリアルタイムで表示します。
//!
//! ## 主な特徴
//! - **軽量・高パフォーマンス**: 最小限のリソース取得（`sysinfo` 最適化）と低 CPU/メモリフットプリント
//! - **オートスナップ吸着**: 画面端 15px に近づくと自動吸着する Win32 連携
//! - **絶対座標アンカー移動**: マウスカーソルと完全に同期した滑らかなドラッグ移動
//! - **リアルタイムスパークライン**: CPU・メモリの直近 30 秒間の推移グラフ表示
//! - **カスタマイズ設定画面**: 背景不透明度、更新間隔、オートスナップ、各項目の表示トグルとウィンドウ幅の動的リサイズ

pub mod config;
pub mod monitor;
pub mod ui;
pub mod win32;

use config::Config;
use eframe::egui;
use monitor::SystemMonitor;
use std::time::Duration;

impl eframe::App for SystemMonitor {
    /// アプリケーション終了時に設定情報（ウィンドウ位置・表示設定）を永続化ストレージに保存します。
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Ok(json) = serde_json::to_string(&self.config) {
            storage.set_string(eframe::APP_KEY, json);
        }
    }

    /// UI の描画および更新ロジックを定義します。
    ///
    /// メトリクスの定期更新、自前ドラッグ移動処理、各表示項目のレイアウト、
    /// および右クリック/ダブルクリックによる設定画面の制御を行います。
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 設定された更新間隔ごとにデータを更新
        let interval = Duration::from_secs_f32(self.config.update_interval_secs);
        if self.last_update.elapsed() >= interval {
            self.update_stats();
            self.last_update = std::time::Instant::now();
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

        let current_width = ui::calculate_window_width(&self.config);
        let mut is_dragging = false;

        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ui, |ui| {
                // テキスト選択を無効化（誤操作・ガタつき防止）
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

                    if drag_label.drag_started() {
                        self.drag_start_win_pos = self.config.pos;
                        #[cfg(target_os = "windows")]
                        {
                            self.drag_start_mouse_pos = win32::native::get_cursor_pos();
                        }
                        #[cfg(not(target_os = "windows"))]
                        {
                            self.drag_start_mouse_pos = ui.input(|i| i.pointer.interact_pos());
                        }
                    }

                    if drag_label.dragged() {
                        is_dragging = true;

                        #[cfg(target_os = "windows")]
                        let current_mouse = win32::native::get_cursor_pos();
                        #[cfg(not(target_os = "windows"))]
                        let current_mouse = ui.input(|i| i.pointer.interact_pos());

                        if let (Some(start_win), Some(start_mouse), Some(curr_mouse)) = (
                            self.drag_start_win_pos,
                            self.drag_start_mouse_pos,
                            current_mouse,
                        ) {
                            let mouse_delta = curr_mouse - start_mouse;
                            #[allow(unused_mut)]
                            let mut new_pos = start_win + mouse_delta;

                            #[cfg(target_os = "windows")]
                            if self.config.enable_snap {
                                let center_x = (new_pos.x + current_width / 2.0) as i32;
                                let center_y = (new_pos.y + 16.0) as i32;
                                if let Some(work_area) =
                                    win32::native::get_monitor_work_area(center_x, center_y)
                                {
                                    new_pos = win32::apply_snap(
                                        new_pos,
                                        egui::vec2(current_width, 32.0),
                                        work_area,
                                    );
                                }
                            }

                            self.config.pos = Some(new_pos);
                            ui.ctx()
                                .send_viewport_cmd(egui::ViewportCommand::OuterPosition(new_pos));
                        }
                    }

                    if drag_label.drag_stopped() {
                        self.drag_start_win_pos = None;
                        self.drag_start_mouse_pos = None;
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
                        ui::draw_trend_line(ui, &self.cpu_history, 100.0);
                        ui.add_space(15.0);
                    }

                    if self.config.show_mem {
                        label_style(ui, "MEM", format!("{:>5.1}%", self.mem_usage), 60.0);
                        ui::draw_trend_line(ui, &self.mem_history, 100.0);
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

            let mut builder = egui::ViewportBuilder::default()
                .with_title("MiSysMon Settings")
                .with_inner_size([300.0, 430.0])
                .with_resizable(false)
                .with_decorations(true)
                .with_always_on_top();

            if let Some(pos) = self.config.pos {
                builder = builder.with_position(egui::pos2(pos.x, pos.y + 36.0));
            }

            ui.ctx().show_viewport_immediate(
                egui::ViewportId::from_hash_of("settings_window"),
                builder,
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
                let new_width = ui::calculate_window_width(&self.config);
                self.last_width = new_width;
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                        new_width, 32.0,
                    )));
            }
        }

        // 幅が変更された場合のみサイズ指定コマンドを送信
        if (self.last_width - current_width).abs() > 0.1 {
            self.last_width = current_width;
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                    current_width,
                    32.0,
                )));
        }

        // ドラッグ中でない場合のみ現在のウィンドウ位置を監視して保存用に更新
        if !is_dragging {
            ui.input(|i| {
                if let Some(rect) = i.viewport().outer_rect {
                    self.config.pos = Some(rect.min);
                }
            });
        }

        ui.ctx().request_repaint_after(Duration::from_secs_f32(
            self.config.update_interval_secs.min(1.0),
        ));
    }
}

/// アプリケーションのエントリーポイント。
///
/// 1. `common_lib` による多重起動防止の確認
/// 2. `eframe::NativeOptions` の設定（サイズ、リサイズ禁止、透過、最前面など）
/// 3. 前回の保存位置のロードと画面内境界チェック
/// 4. UI イベントループの起動
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
            // 保存された位置がある場合は適用
            let config: Config = cc
                .storage
                .and_then(|s| s.get_string(storage_name))
                .and_then(|json| serde_json::from_str(&json).ok())
                .unwrap_or_default();

            let monitor = SystemMonitor::new(cc);
            let initial_width = ui::calculate_window_width(&monitor.config);

            if let Some(pos) = config.pos {
                // ウィンドウ位置が有効なモニターの表示範囲内にあるか検証
                #[cfg(target_os = "windows")]
                let is_valid =
                    win32::native::is_position_on_any_monitor(pos.x as i32, pos.y as i32);
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
