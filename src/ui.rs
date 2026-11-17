//! # UI レンダリングおよびレイアウト計算モジュール (`ui`)
//!
//! スパークライン（ミニトレンドライン）のカスタム描画や、
//! 設定に基づいたウィンドウ必要幅の動的計算関数を提供します。

use crate::config::Config;
use eframe::egui;

/// 通常時のスリムバーの高さ（px）。
pub const BAR_HEIGHT: f32 = 32.0;

/// 設定パネル展開時の最小ウィンドウ幅（px）。
pub const SETTINGS_MIN_WIDTH: f32 = 300.0;

/// 設定パネル展開時のウィンドウ高さ（px）。
pub const SETTINGS_EXPANDED_HEIGHT: f32 = 475.0;

/// 現在の設定および設定パネルの開閉状態に基づいて、必要なメインウィンドウのサイズ `(幅, 高さ)` を計算します。
///
/// - 通常時 (`settings_open == false`): `(calculate_window_width(config), BAR_HEIGHT)`
/// - 設定展開時 (`settings_open == true`): `(calculate_window_width(config).max(SETTINGS_MIN_WIDTH), SETTINGS_EXPANDED_HEIGHT)`
pub fn calculate_window_size(config: &Config, settings_open: bool) -> egui::Vec2 {
    let base_width = calculate_window_width(config);
    if settings_open {
        egui::vec2(base_width.max(SETTINGS_MIN_WIDTH), SETTINGS_EXPANDED_HEIGHT)
    } else {
        egui::vec2(base_width, BAR_HEIGHT)
    }
}

/// 現在の設定（各項目の表示/非表示フラグ）に基づいて、必要なメインウィンドウの横幅（px）を計算します。
///
/// 不要な余白を削減し、表示項目数に応じた最適なウィンドウサイズを提供します。
/// 最小幅は `200.0` ピクセルに制限されます。
///
/// # 計算内訳
/// - 基本幅（マージン、ドラッググリップ、終了ボタン、スペーシング）: 70.0 px
/// - CPU 表示 (`show_cpu`): +185.0 px
/// - メモリ表示 (`show_mem`): +150.0 px
/// - ネットワーク表示 (`show_net`): +185.0 px
/// - ディスク容量表示 (`show_disk`): +160.0 px
/// - ディスク I/O 表示 (`show_io`): +175.0 px
/// - バージョン番号表示 (`show_version`): +90.0 px
/// - 時計表示 (`show_clock`): +195.0 px
///
/// # 引数
/// - `config`: アプリケーションの現在設定への参照
///
/// # 戻り値
/// - 計算されたウィンドウの幅（f32）
pub fn calculate_window_width(config: &Config) -> f32 {
    let mut width: f32 = 70.0; // 左右マージン + グリップ + 終了ボタン + spacing

    if config.show_cpu {
        width += 185.0;
    }
    if config.show_mem {
        width += 150.0;
    }
    if config.show_net {
        width += 185.0;
    }
    if config.show_disk {
        width += 160.0;
    }
    if config.show_io {
        width += 175.0;
    }
    if config.show_version {
        width += 90.0;
    }
    if config.show_clock {
        width += 195.0;
    }

    width.max(200.0) // 最小幅を 200px に制限
}

/// 直近のメトリクス履歴を、高さ 12px のミニスパークライン（折れ線グラフ）として描画します。
///
/// # 引数
/// - `ui`: egui の描画コンテキスト
/// - `history`: 直近のメトリクス値のスライス（最大 30 点）
/// - `max_val`: グラフの縦軸の最大値（CPU/メモリの場合は `100.0`）
pub fn draw_trend_line(ui: &mut egui::Ui, history: &[f32], max_val: f32) {
    let desired_size = egui::vec2(40.0, 12.0);
    let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();

        // グラフ背景の半透明矩形
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
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 255, 255)), // シアン色
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_window_width_all_enabled() {
        let config = Config::default(); // 全項目 true
        let width = calculate_window_width(&config);
        // 70 + 185 + 150 + 185 + 160 + 175 + 90 + 195 = 1210.0
        assert_eq!(width, 1210.0);
    }

    #[test]
    fn test_calculate_window_width_all_disabled_minimum_clamping() {
        let config = Config {
            pos: None,
            update_interval_secs: 1.0,
            opacity: 0.78,
            enable_snap: true,
            show_cpu: false,
            show_mem: false,
            show_net: false,
            show_disk: false,
            show_io: false,
            show_version: false,
            show_clock: false,
        };
        let width = calculate_window_width(&config);
        // 基本幅 70.0 だが、最小幅 200.0 にクランプされる
        assert_eq!(width, 200.0);
    }

    #[test]
    fn test_calculate_window_width_custom_selection() {
        let config = Config {
            show_cpu: true, // 185
            show_mem: false,
            show_net: false,
            show_disk: false,
            show_io: false,
            show_version: false,
            show_clock: true, // 195
            ..Default::default()
        };
        // 70 + 185 + 195 = 450.0

        let width = calculate_window_width(&config);
        assert_eq!(width, 450.0);
    }

    #[test]
    fn test_calculate_window_size_settings_toggle() {
        let config = Config {
            show_cpu: false,
            show_mem: false,
            show_net: false,
            show_disk: false,
            show_io: false,
            show_version: false,
            show_clock: false,
            ..Default::default()
        };

        // 通常時は最小幅 200.0 x 高さ 32.0
        let normal_size = calculate_window_size(&config, false);
        assert_eq!(normal_size, egui::vec2(200.0, BAR_HEIGHT));

        // 設定展開時は最小幅 300.0 x 展開高さ 475.0 に拡大される
        let expanded_size = calculate_window_size(&config, true);
        assert_eq!(
            expanded_size,
            egui::vec2(SETTINGS_MIN_WIDTH, SETTINGS_EXPANDED_HEIGHT)
        );
    }
}
