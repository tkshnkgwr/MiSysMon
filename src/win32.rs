//! # Win32 API 連携およびウィンドウ吸着モジュール (`win32`)
//!
//! Windows プラットフォーム固有の機能（マウスカーソル絶対座標取得、
//! モニター領域判定、作業領域取得、および画面端オートスナップ計算）を提供します。

use eframe::egui;

#[cfg(target_os = "windows")]
#[allow(clippy::upper_case_acronyms)]
pub mod native {
    use eframe::egui;
    use std::ffi::c_void;

    /// Win32 モニターハンドル型。
    type HMONITOR = *mut c_void;

    /// 2次元座標を表す Win32 構造体。
    #[repr(C)]
    pub struct POINT {
        pub x: i32,
        pub y: i32,
    }

    /// 最も近いモニターを取得するための Win32 定数。
    pub const MONITOR_DEFAULTTONEAREST: u32 = 2;

    /// 矩形領域（左、上、右、下）を表す Win32 構造体。
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct RECT {
        pub left: i32,
        pub top: i32,
        pub right: i32,
        pub bottom: i32,
    }

    /// モニターの詳細情報を格納する Win32 構造体。
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
        fn GetCursorPos(lp_point: *mut POINT) -> i32;
    }

    /// 現在のマウス絶対画面座標 `(x, y)` を取得します。
    ///
    /// # 戻り値
    /// - 取得成功時: `Some(egui::Pos2)`（ピクセル座標）
    /// - 失敗時: `None`
    pub fn get_cursor_pos() -> Option<egui::Pos2> {
        unsafe {
            let mut pt = POINT { x: 0, y: 0 };
            if GetCursorPos(&mut pt) != 0 {
                Some(egui::Pos2::new(pt.x as f32, pt.y as f32))
            } else {
                None
            }
        }
    }

    /// 指定したスクリーン座標 `(x, y)` が現在接続されているいずれかのモニターの表示領域内にあるかを判定します。
    ///
    /// マルチモニター環境の切断時や解像度変更時に、保存された座標が画面外に消えるのを防止するために使用します。
    ///
    /// # 引数
    /// - `x`: スクリーン X 座標
    /// - `y`: スクリーン Y 座標
    ///
    /// # 戻り値
    /// - 表示領域内であれば `true`、範囲外なら `false`
    pub fn is_position_on_any_monitor(x: i32, y: i32) -> bool {
        unsafe {
            let pt = POINT { x, y };
            let h_monitor = MonitorFromPoint(pt, 0); // MONITOR_DEFAULTTONULL (0)
            !h_monitor.is_null()
        }
    }

    /// 指定した座標 `(x, y)` に対応するモニターの作業領域（タスクバー等を除く利用可能領域）を取得します。
    ///
    /// # 引数
    /// - `x`: スクリーン X 座標
    /// - `y`: スクリーン Y 座標
    ///
    /// # 戻り値
    /// - 取得成功時: `Some(RECT)`（作業領域の矩形）
    /// - 失敗時: `None`
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

/// 矩形領域を表す共通構造体（非 Windows プラットフォームおよびテスト用）。
#[cfg(not(target_os = "windows"))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RectArea {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[cfg(target_os = "windows")]
pub type WorkAreaRect = native::RECT;

#[cfg(not(target_os = "windows"))]
pub type WorkAreaRect = RectArea;

/// ウィンドウの位置をモニター作業領域の画面端に自動吸着（オートスナップ）させます。
///
/// ウィンドウの上下左右の端が作業領域の境界線から `threshold`（15 ピクセル）以内に近づいた場合、
/// 境界線ぴったりに座標を補正します。
///
/// # 引数
/// - `pos`: 移動候補となるウィンドウの左上座標 `(x, y)`
/// - `size`: ウィンドウのサイズ `(幅, 高さ)`
/// - `work_area`: モニターの作業領域矩形（タスクバーを除いた表示可能範囲）
///
/// # 戻り値
/// - 吸着補正後の左上座標 `egui::Pos2`
pub fn apply_snap(pos: egui::Pos2, size: egui::Vec2, work_area: WorkAreaRect) -> egui::Pos2 {
    let mut x = pos.x;
    let mut y = pos.y;
    let width = size.x;
    let height = size.y;

    let threshold = 15.0; // 吸着のしきい値（ピクセル単位）

    // 左端の吸着判定
    if (x - work_area.left as f32).abs() < threshold {
        x = work_area.left as f32;
    }
    // 右端の吸着判定
    else if ((x + width) - work_area.right as f32).abs() < threshold {
        x = work_area.right as f32 - width;
    }

    // 上端の吸着判定
    if (y - work_area.top as f32).abs() < threshold {
        y = work_area.top as f32;
    }
    // 下端の吸着判定
    else if ((y + height) - work_area.bottom as f32).abs() < threshold {
        y = work_area.bottom as f32 - height;
    }

    egui::Pos2::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_work_area() -> WorkAreaRect {
        WorkAreaRect {
            left: 0,
            top: 0,
            right: 1920,
            bottom: 1040, // 1080p でタスクバー 40px を除いた作業領域
        }
    }

    #[test]
    fn test_apply_snap_left_edge() {
        let work_area = sample_work_area();
        let size = egui::vec2(500.0, 32.0);

        // 左端から 10px の位置（しきい値 15px 未満） -> 0px に吸着
        let pos = egui::pos2(10.0, 200.0);
        let snapped = apply_snap(pos, size, work_area);
        assert_eq!(snapped.x, 0.0);
        assert_eq!(snapped.y, 200.0);

        // 左端から -8px の位置（しきい値 15px 未満） -> 0px に吸着
        let pos_neg = egui::pos2(-8.0, 200.0);
        let snapped_neg = apply_snap(pos_neg, size, work_area);
        assert_eq!(snapped_neg.x, 0.0);
    }

    #[test]
    fn test_apply_snap_right_edge() {
        let work_area = sample_work_area();
        let size = egui::vec2(500.0, 32.0);

        // 右端 (1920 - 500 = 1420) に対して 1425 (差 5px) -> 1420 に吸着
        let pos = egui::pos2(1425.0, 200.0);
        let snapped = apply_snap(pos, size, work_area);
        assert_eq!(snapped.x, 1420.0);
        assert_eq!(snapped.y, 200.0);
    }

    #[test]
    fn test_apply_snap_top_edge() {
        let work_area = sample_work_area();
        let size = egui::vec2(500.0, 32.0);

        // 上端から 12px -> 0px に吸着
        let pos = egui::pos2(300.0, 12.0);
        let snapped = apply_snap(pos, size, work_area);
        assert_eq!(snapped.x, 300.0);
        assert_eq!(snapped.y, 0.0);
    }

    #[test]
    fn test_apply_snap_bottom_edge() {
        let work_area = sample_work_area();
        let size = egui::vec2(500.0, 32.0);

        // 下端 (1040 - 32 = 1008) に対して 1000 (差 8px) -> 1008 に吸着
        let pos = egui::pos2(300.0, 1000.0);
        let snapped = apply_snap(pos, size, work_area);
        assert_eq!(snapped.x, 300.0);
        assert_eq!(snapped.y, 1008.0);
    }

    #[test]
    fn test_apply_snap_no_snap_when_outside_threshold() {
        let work_area = sample_work_area();
        let size = egui::vec2(500.0, 32.0);

        // しきい値 15px を超える位置（左右・上下とも中央付近） -> そのまま維持
        let pos = egui::pos2(400.0, 300.0);
        let snapped = apply_snap(pos, size, work_area);
        assert_eq!(snapped.x, 400.0);
        assert_eq!(snapped.y, 300.0);
    }

    #[test]
    fn test_apply_snap_corner_snap() {
        let work_area = sample_work_area();
        let size = egui::vec2(500.0, 32.0);

        // 左上コーナー（左 5px, 上 5px） -> (0, 0) に同時吸着
        let pos = egui::pos2(5.0, 5.0);
        let snapped = apply_snap(pos, size, work_area);
        assert_eq!(snapped.x, 0.0);
        assert_eq!(snapped.y, 0.0);
    }
}
