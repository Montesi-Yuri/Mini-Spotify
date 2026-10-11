use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::time::Duration;

use tauri::{AppHandle, LogicalSize, Manager, PhysicalPosition, PhysicalSize, WindowEvent};

use crate::tray;

/// How the widget arranges its content, chosen by the window's width.
/// The thresholds mirror the CSS media queries in `Player.css`
/// (compact is `max-width: 480px`, extra-compact is `max-width: 160px`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    Full,
    Compact,
    ExtraCompact,
}

impl Layout {
    pub const ALL: [Layout; 3] = [Layout::Full, Layout::Compact, Layout::ExtraCompact];

    /// Standard logical window size (width, height) for this layout's preset.
    pub fn size(self) -> (f64, f64) {
        match self {
            Layout::Full => (800.0, 100.0),
            Layout::Compact => (400.0, 100.0),
            Layout::ExtraCompact => (100.0, 100.0),
        }
    }

    /// The layout a window of this logical width displays.
    pub fn from_width(width: f64) -> Layout {
        if width <= 160.0 {
            Layout::ExtraCompact
        } else if width <= 480.0 {
            Layout::Compact
        } else {
            Layout::Full
        }
    }

    pub fn next(self) -> Layout {
        match self {
            Layout::Full => Layout::Compact,
            Layout::Compact => Layout::ExtraCompact,
            Layout::ExtraCompact => Layout::Full,
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Layout::Full => "layout-full",
            Layout::Compact => "layout-compact",
            Layout::ExtraCompact => "layout-extra-compact",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Layout::Full => "Full",
            Layout::Compact => "Compact",
            Layout::ExtraCompact => "Extra-compact",
        }
    }
}

/// An axis-aligned rectangle in physical pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.x + self.w && py >= self.y && py < self.y + self.h
    }

    fn right(&self) -> i32 {
        self.x + self.w
    }

    fn bottom(&self) -> i32 {
        self.y + self.h
    }
}

/// The monitor holding the window's centre, falling back to the first (primary) one.
fn monitor_for<'a>(window: Rect, monitors: &'a [Rect]) -> Option<&'a Rect> {
    let (cx, cy) = (window.x + window.w / 2, window.y + window.h / 2);
    monitors
        .iter()
        .find(|m| m.contains(cx, cy))
        .or(monitors.first())
}

/// Puts `window` fully inside a monitor, shrinking its width if the monitor is narrower.
/// With no monitors the window is left alone.
pub fn clamp_to_monitors(window: Rect, monitors: &[Rect]) -> Rect {
    let Some(m) = monitor_for(window, monitors) else {
        return window;
    };
    let w = window.w.min(m.w);
    let x = window.x.min(m.right() - w).max(m.x);
    // Height is fixed per layout, so only the position is clamped vertically.
    let y = window.y.min(m.bottom() - window.h).max(m.y);
    Rect { x, y, w, h: window.h }
}

/// Docks the window flush to a monitor edge when it is within `threshold` of it.
pub fn snap_to_edges(window: Rect, monitors: &[Rect], threshold: i32) -> (i32, i32) {
    let Some(m) = monitor_for(window, monitors) else {
        return (window.x, window.y);
    };
    let snap = |pos: i32, size: i32, lo: i32, hi: i32| {
        if (pos - lo).abs() <= threshold {
            lo
        } else if (pos + size - hi).abs() <= threshold {
            hi - size
        } else {
            pos
        }
    };
    (
        snap(window.x, window.w, m.x, m.right()),
        snap(window.y, window.h, m.y, m.bottom()),
    )
}

/// Stretches the window across its monitor, keeping its row and height.
pub fn fit_width(window: Rect, monitors: &[Rect]) -> Rect {
    match monitor_for(window, monitors) {
        Some(m) => Rect { x: m.x, w: m.w, ..window },
        None => window,
    }
}

fn work_rect(m: &tauri::Monitor) -> Rect {
    let r = m.work_area();
    Rect {
        x: r.position.x,
        y: r.position.y,
        w: r.size.width as i32,
        h: r.size.height as i32,
    }
}

/// Usable area of every monitor, primary first so it is the fallback.
fn monitor_rects(window: &tauri::WebviewWindow) -> Vec<Rect> {
    let mut monitors: Vec<Rect> = Vec::new();
    if let Ok(Some(p)) = window.primary_monitor() {
        monitors.push(work_rect(&p));
    }
    for m in window.available_monitors().unwrap_or_default() {
        let r = work_rect(&m);
        if !monitors.contains(&r) {
            monitors.push(r);
        }
    }
    monitors
}

fn main_window(app: &AppHandle) -> Option<tauri::WebviewWindow> {
    app.get_webview_window("main")
}

fn window_rect(window: &tauri::WebviewWindow) -> Option<Rect> {
    let (pos, size) = (window.outer_position().ok()?, window.outer_size().ok()?);
    Some(Rect {
        x: pos.x,
        y: pos.y,
        w: size.width as i32,
        h: size.height as i32,
    })
}

fn place(window: &tauri::WebviewWindow, from: Rect, to: Rect) {
    if (to.w, to.h) != (from.w, from.h) {
        let _ = window.set_size(PhysicalSize::new(to.w as u32, to.h as u32));
    }
    if (to.x, to.y) != (from.x, from.y) {
        let _ = window.set_position(PhysicalPosition::new(to.x, to.y));
    }
}

/// The layout the window currently displays.
pub fn current(app: &AppHandle) -> Layout {
    let width = main_window(app)
        .and_then(|w| Some(w.inner_size().ok()?.width as f64 / w.scale_factor().ok()?))
        .unwrap_or(Layout::Full.size().0);
    Layout::from_width(width)
}

/// Resizes to the layout's standard size, keeping the top-left corner, and keeps it on screen.
pub fn apply(app: &AppHandle, layout: Layout) {
    if let Some(window) = main_window(app) {
        let (w, h) = layout.size();
        let _ = window.set_size(LogicalSize::new(w, h));
        settle(app);
    }
    tray::sync_layout(app, layout);
}

/// Stretches the widget across its monitor.
pub fn fit_to_screen(app: &AppHandle) {
    let Some(window) = main_window(app) else {
        return;
    };
    if let Some(current) = window_rect(&window) {
        place(&window, current, fit_width(current, &monitor_rects(&window)));
    }
}

/// Keeps the window on a visible monitor, then docks it to any edge it is next to.
fn settle(app: &AppHandle) {
    let Some(window) = main_window(app) else {
        return;
    };
    let Some(current) = window_rect(&window) else {
        return;
    };
    let monitors = monitor_rects(&window);
    let clamped = clamp_to_monitors(current, &monitors);
    let threshold = (SNAP_DISTANCE * window.scale_factor().unwrap_or(1.0)) as i32;
    let (x, y) = snap_to_edges(clamped, &monitors, threshold);
    place(&window, current, Rect { x, y, ..clamped });
}

/// How close (logical px) to a monitor edge the widget must be dropped to dock to it.
const SNAP_DISTANCE: f64 = 16.0;
/// Wait for the drag to stop before snapping, so the widget doesn't fight the cursor.
const SETTLE_DELAY: Duration = Duration::from_millis(250);

/// Start-up and ongoing window behaviour: keep the tray in sync with the width,
/// and settle the window (clamp + snap) once it stops moving. The first settle
/// also covers the position restored by the window-state plugin.
pub fn init(app: &AppHandle) {
    let Some(window) = main_window(app) else {
        return;
    };
    let handle = app.clone();
    let moves = Arc::new(AtomicU64::new(0));
    window.on_window_event(move |event| match event {
        WindowEvent::Moved(_) => {
            let id = moves.fetch_add(1, Ordering::SeqCst) + 1;
            let (handle, moves) = (handle.clone(), moves.clone());
            // Another move before the delay ends supersedes this one.
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(SETTLE_DELAY).await;
                if moves.load(Ordering::SeqCst) == id {
                    settle(&handle);
                }
            });
        }
        WindowEvent::Resized(_) => tray::sync_layout(&handle, current(&handle)),
        _ => {}
    });
    settle(app);
}

/// Double-click on the widget cycles Full → Compact → Extra-compact → Full.
#[tauri::command]
pub fn cycle_layout(app: AppHandle) {
    apply(&app, current(&app).next());
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCREEN: Rect = Rect { x: 0, y: 0, w: 1920, h: 1080 };

    fn win(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { x, y, w, h }
    }

    #[test]
    fn layout_cycles_through_all_three() {
        assert_eq!(Layout::Full.next(), Layout::Compact);
        assert_eq!(Layout::Compact.next(), Layout::ExtraCompact);
        assert_eq!(Layout::ExtraCompact.next(), Layout::Full);
    }

    #[test]
    fn width_selects_layout() {
        assert_eq!(Layout::from_width(100.0), Layout::ExtraCompact);
        assert_eq!(Layout::from_width(160.0), Layout::ExtraCompact);
        assert_eq!(Layout::from_width(161.0), Layout::Compact);
        assert_eq!(Layout::from_width(480.0), Layout::Compact);
        assert_eq!(Layout::from_width(481.0), Layout::Full);
    }

    #[test]
    fn presets_display_their_own_layout() {
        for layout in Layout::ALL {
            assert_eq!(Layout::from_width(layout.size().0), layout);
        }
    }

    #[test]
    fn visible_window_is_left_alone() {
        let w = win(100, 50, 800, 100);
        assert_eq!(clamp_to_monitors(w, &[SCREEN]), w);
    }

    #[test]
    fn window_hanging_off_right_edge_is_pulled_in() {
        assert_eq!(
            clamp_to_monitors(win(1700, 50, 800, 100), &[SCREEN]),
            win(1120, 50, 800, 100)
        );
    }

    #[test]
    fn window_on_removed_monitor_falls_back_to_primary() {
        // Saved on a second monitor that is no longer connected.
        assert_eq!(
            clamp_to_monitors(win(2500, 300, 800, 100), &[SCREEN]),
            win(1120, 300, 800, 100)
        );
        assert_eq!(
            clamp_to_monitors(win(-2000, -400, 800, 100), &[SCREEN]),
            win(0, 0, 800, 100)
        );
    }

    #[test]
    fn window_wider_than_monitor_shrinks_to_fit() {
        let small = Rect { x: 0, y: 0, w: 700, h: 500 };
        assert_eq!(
            clamp_to_monitors(win(50, 10, 800, 100), &[small]),
            win(0, 10, 700, 100)
        );
    }

    #[test]
    fn window_stays_on_its_own_monitor() {
        let second = Rect { x: 1920, y: 0, w: 1920, h: 1080 };
        let w = win(2000, 100, 800, 100);
        assert_eq!(clamp_to_monitors(w, &[SCREEN, second]), w);
    }

    #[test]
    fn no_monitors_leaves_window() {
        let w = win(5, 6, 800, 100);
        assert_eq!(clamp_to_monitors(w, &[]), w);
        assert_eq!(snap_to_edges(w, &[], 16), (5, 6));
        assert_eq!(fit_width(w, &[]), w);
    }

    #[test]
    fn snaps_to_near_edges_and_corners() {
        // Near top-left corner.
        assert_eq!(snap_to_edges(win(10, 12, 800, 100), &[SCREEN], 16), (0, 0));
        // Near bottom-right corner.
        assert_eq!(
            snap_to_edges(win(1110, 970, 800, 100), &[SCREEN], 16),
            (1120, 980)
        );
    }

    #[test]
    fn does_not_snap_when_far_from_edges() {
        assert_eq!(snap_to_edges(win(100, 100, 800, 100), &[SCREEN], 16), (100, 100));
    }

    #[test]
    fn fit_width_spans_monitor_keeping_row_and_height() {
        assert_eq!(
            fit_width(win(300, 40, 400, 100), &[SCREEN]),
            win(0, 40, 1920, 100)
        );
    }
}
