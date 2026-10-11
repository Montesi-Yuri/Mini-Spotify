use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, LogicalSize, Manager, PhysicalPosition};

use crate::tray;

/// How the widget arranges its content. The window size per layout is what
/// triggers the matching CSS media query in `Player.css`, so keep the two in sync:
/// compact is `max-width: 480px`, extra-compact is `max-width: 160px`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Layout {
    #[default]
    Full,
    Compact,
    ExtraCompact,
}

impl Layout {
    pub const ALL: [Layout; 3] = [Layout::Full, Layout::Compact, Layout::ExtraCompact];

    /// Logical window size (width, height).
    pub fn size(self) -> (f64, f64) {
        match self {
            Layout::Full => (800.0, 100.0),
            Layout::Compact => (400.0, 100.0),
            Layout::ExtraCompact => (100.0, 100.0),
        }
    }

    pub fn next(self) -> Layout {
        match self {
            Layout::Full => Layout::Compact,
            Layout::Compact => Layout::ExtraCompact,
            Layout::ExtraCompact => Layout::Full,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub layout: Layout,
}

impl Settings {
    /// Missing or unreadable settings fall back to defaults.
    pub fn load(path: &Path) -> Settings {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(path, serde_json::to_string_pretty(self)?)
    }
}

fn settings_path(app: &AppHandle) -> Option<PathBuf> {
    app.path().app_data_dir().ok().map(|d| d.join("settings.json"))
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
}

/// Returns the top-left position that puts `window` fully inside a monitor.
/// Uses the monitor holding the window's centre, falling back to the first
/// (primary) one. With no monitors the position is left alone.
pub fn clamp_to_monitors(window: Rect, monitors: &[Rect]) -> (i32, i32) {
    let (cx, cy) = (window.x + window.w / 2, window.y + window.h / 2);
    let Some(m) = monitors
        .iter()
        .find(|m| m.contains(cx, cy))
        .or(monitors.first())
    else {
        return (window.x, window.y);
    };
    // A window larger than the monitor sticks to its top-left corner.
    let x = window.x.min(m.x + m.w - window.w).max(m.x);
    let y = window.y.min(m.y + m.h - window.h).max(m.y);
    (x, y)
}

/// Moves the main window back onto a visible monitor if it is (partly) off-screen.
pub fn clamp_main_window(app: &AppHandle) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    let (Ok(pos), Ok(size)) = (window.outer_position(), window.outer_size()) else {
        return;
    };
    let mut monitors: Vec<Rect> = Vec::new();
    // Primary first so it is the fallback.
    if let Ok(Some(p)) = window.primary_monitor() {
        monitors.push(work_rect(&p));
    }
    for m in window.available_monitors().unwrap_or_default() {
        let r = work_rect(&m);
        if !monitors.contains(&r) {
            monitors.push(r);
        }
    }
    let current = Rect {
        x: pos.x,
        y: pos.y,
        w: size.width as i32,
        h: size.height as i32,
    };
    let (x, y) = clamp_to_monitors(current, &monitors);
    if (x, y) != (pos.x, pos.y) {
        let _ = window.set_position(PhysicalPosition::new(x, y));
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

/// Resizes the window for `layout`, keeping its top-left corner, and keeps it on screen.
fn resize_window(app: &AppHandle, layout: Layout) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    let (w, h) = layout.size();
    let _ = window.set_size(LogicalSize::new(w, h));
    clamp_main_window(app);
}

/// Current layout, managed as Tauri state.
pub struct LayoutState(pub std::sync::Mutex<Layout>);

pub fn current(app: &AppHandle) -> Layout {
    *app.state::<LayoutState>().0.lock().unwrap()
}

/// Switches layout: resize, persist, and sync the tray radio items.
pub fn apply(app: &AppHandle, layout: Layout) {
    *app.state::<LayoutState>().0.lock().unwrap() = layout;
    resize_window(app, layout);
    tray::sync_layout(app, layout);
    if let Some(path) = settings_path(app) {
        if let Err(e) = (Settings { layout }).save(&path) {
            eprintln!("failed to save settings: {e}");
        }
    }
}

/// Reads the saved layout and applies it to the window. Call once at startup.
pub fn init(app: &AppHandle) {
    let layout = settings_path(app)
        .map(|p| Settings::load(&p).layout)
        .unwrap_or_default();
    app.manage(LayoutState(std::sync::Mutex::new(layout)));
    resize_window(app, layout);
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
    fn sizes_match_css_breakpoints() {
        assert!(Layout::Compact.size().0 <= 480.0 && Layout::Compact.size().0 > 160.0);
        assert!(Layout::ExtraCompact.size().0 <= 160.0);
        assert!(Layout::Full.size().0 > 480.0);
    }

    #[test]
    fn visible_window_is_left_alone() {
        assert_eq!(clamp_to_monitors(win(100, 50, 800, 100), &[SCREEN]), (100, 50));
    }

    #[test]
    fn window_hanging_off_right_edge_is_pulled_in() {
        assert_eq!(clamp_to_monitors(win(1700, 50, 800, 100), &[SCREEN]), (1120, 50));
    }

    #[test]
    fn window_on_removed_monitor_falls_back_to_primary() {
        // Saved on a second monitor that is no longer connected.
        assert_eq!(clamp_to_monitors(win(2500, 300, 800, 100), &[SCREEN]), (1120, 300));
        assert_eq!(clamp_to_monitors(win(-2000, -400, 800, 100), &[SCREEN]), (0, 0));
    }

    #[test]
    fn window_stays_on_its_own_monitor() {
        let second = Rect { x: 1920, y: 0, w: 1920, h: 1080 };
        assert_eq!(
            clamp_to_monitors(win(2000, 100, 800, 100), &[SCREEN, second]),
            (2000, 100)
        );
    }

    #[test]
    fn no_monitors_leaves_position() {
        assert_eq!(clamp_to_monitors(win(5, 6, 800, 100), &[]), (5, 6));
    }

    #[test]
    fn settings_round_trip_and_defaults() {
        let dir = std::env::temp_dir().join(format!("mw-settings-{}", std::process::id()));
        let path = dir.join("settings.json");
        assert_eq!(Settings::load(&path).layout, Layout::Full);
        Settings { layout: Layout::ExtraCompact }.save(&path).unwrap();
        assert_eq!(Settings::load(&path).layout, Layout::ExtraCompact);
        std::fs::write(&path, "not json").unwrap();
        assert_eq!(Settings::load(&path).layout, Layout::Full);
        let _ = std::fs::remove_dir_all(dir);
    }
}
