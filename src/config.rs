//! # 設定管理モジュール (`config`)
//!
//! アプリケーションの表示オプション、ウィンドウの不透明度、更新間隔、
//! および終了時のウィンドウ座標などの設定情報を管理します。
//!
//! 設定は JSON 形式で永続化ストレージ（`eframe::Storage`）に保存・復元されます。

use eframe::egui;
use serde::{Deserialize, Serialize};

/// アプリケーションの設定情報を保持する構造体。
///
/// ウィンドウの位置や各メトリクス項目の表示/非表示フラグなど、
/// 次回起動時に復元したい情報を保持します。
///
/// JSON シリアライズ/デシリアライズに対応しており、未知のフィールドや
/// 欠落したフィールドがある場合はデフォルト値が自動補完されます。
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct Config {
    /// 終了時のウィンドウの左上座標（スクリーン座標系）。
    ///
    /// `None` の場合は OS のデフォルト位置（または画面中央）に配置されます。
    pub pos: Option<egui::Pos2>,

    /// メトリクスの更新間隔（秒単位）。
    ///
    /// スライダーにより 0.5 秒から 5.0 秒の範囲で設定可能です。デフォルトは `1.0` 秒。
    pub update_interval_secs: f32,

    /// メインウィンドウの背景不透明度 (アルファ値)。
    ///
    /// `0.3`（高透明）から `1.0`（完全不透明）の範囲で設定可能です。デフォルトは `0.78`。
    pub opacity: f32,

    /// ウィンドウの画面端吸着（オートスナップ）を有効化するかどうかのフラグ。
    ///
    /// 有効な場合、画面端から 15 ピクセル以内に近づくと自動的にピタッと吸着します。
    /// デフォルトは `true`。
    pub enable_snap: bool,

    /// CPU 使用率および CPU 温度の表示フラグ。デフォルトは `true`。
    pub show_cpu: bool,

    /// メモリ使用率の表示フラグ。デフォルトは `true`。
    pub show_mem: bool,

    /// ネットワークトラフィック（上り/下り速度）の表示フラグ。デフォルトは `true`。
    pub show_net: bool,

    /// ディスク使用量（使用量/総容量）の表示フラグ。デフォルトは `true`。
    pub show_disk: bool,

    /// ディスク I/O 速度（読み込み/書き込み）の表示フラグ。デフォルトは `true`。
    pub show_io: bool,

    /// アプリケーションバージョン番号（`vX.Y.Z`）の表示フラグ。デフォルトは `true`。
    pub show_version: bool,

    /// 現在日時の時計表示フラグ。デフォルトは `true`。
    pub show_clock: bool,
}

impl Default for Config {
    /// アプリケーションの既定（デフォルト）設定を返します。
    ///
    /// すべての表示項目が有効（`true`）、更新間隔 1.0 秒、不透明度 0.78、
    /// オートスナップ有効、位置は未指定（`None`）となります。
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default_values() {
        let config = Config::default();
        assert!(config.pos.is_none());
        assert!((config.update_interval_secs - 1.0).abs() < f32::EPSILON);
        assert!((config.opacity - 0.78).abs() < 0.001);
        assert!(config.enable_snap);
        assert!(config.show_cpu);
        assert!(config.show_mem);
        assert!(config.show_net);
        assert!(config.show_disk);
        assert!(config.show_io);
        assert!(config.show_version);
        assert!(config.show_clock);
    }

    #[test]
    fn test_config_json_serialization_roundtrip() {
        let config = Config {
            pos: Some(egui::Pos2::new(120.0, 240.0)),
            opacity: 0.5,
            show_clock: false,
            show_version: false,
            ..Default::default()
        };

        let json = serde_json::to_string(&config).expect("シリアライズに成功すること");
        let deserialized: Config =
            serde_json::from_str(&json).expect("デシリアライズに成功すること");

        assert_eq!(config, deserialized);
        assert_eq!(deserialized.pos, Some(egui::Pos2::new(120.0, 240.0)));
        assert!(!deserialized.show_clock);
        assert!(!deserialized.show_version);
    }

    #[test]
    fn test_config_json_partial_fallback() {
        // 一部の項目のみを含む JSON からでも、未定義項目がデフォルト値で補完されることを検証
        let partial_json = r#"{"opacity": 0.9, "show_cpu": false}"#;
        let config: Config =
            serde_json::from_str(partial_json).expect("部分的なJSONのパースに成功すること");

        assert!((config.opacity - 0.9).abs() < 0.001);
        assert!(!config.show_cpu);
        assert!(config.show_mem); // デフォルトで true
        assert!(config.enable_snap); // デフォルトで true
        assert!(config.pos.is_none());
    }
}
