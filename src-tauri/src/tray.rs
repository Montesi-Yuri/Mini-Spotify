use tauri::{
    menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
use tauri_plugin_autostart::ManagerExt;

use crate::layout::{self, Layout};

const TOGGLE: &str = "toggle";
const AUTOSTART: &str = "autostart";
const QUIT: &str = "quit";

/// Checkable items, kept so their checkmarks follow changes made elsewhere.
struct TrayItems {
    layouts: Vec<(Layout, CheckMenuItem<tauri::Wry>)>,
    autostart: CheckMenuItem<tauri::Wry>,
}

fn layout_id(layout: Layout) -> &'static str {
    match layout {
        Layout::Full => "layout-full",
        Layout::Compact => "layout-compact",
        Layout::ExtraCompact => "layout-extra-compact",
    }
}

fn layout_label(layout: Layout) -> &'static str {
    match layout {
        Layout::Full => "Full",
        Layout::Compact => "Compact",
        Layout::ExtraCompact => "Extra-compact",
    }
}

pub fn init(app: &AppHandle) -> tauri::Result<()> {
    let current = layout::current(app);

    let toggle = MenuItem::with_id(app, TOGGLE, "Show / Hide", true, None::<&str>)?;
    let items: Vec<_> = Layout::ALL
        .iter()
        .map(|&l| {
            CheckMenuItem::with_id(app, layout_id(l), layout_label(l), true, l == current, None::<&str>)
                .map(|item| (l, item))
        })
        .collect::<tauri::Result<_>>()?;
    let autostart_on = app.autolaunch().is_enabled().unwrap_or(false);
    let autostart = CheckMenuItem::with_id(app, AUTOSTART, "Start at login", true, autostart_on, None::<&str>)?;
    let quit = MenuItem::with_id(app, QUIT, "Quit", true, None::<&str>)?;

    let sep = || PredefinedMenuItem::separator(app);
    let menu = Menu::new(app)?;
    menu.append(&toggle)?;
    menu.append(&sep()?)?;
    for (_, item) in &items {
        menu.append(item)?;
    }
    menu.append(&sep()?)?;
    menu.append(&autostart)?;
    menu.append(&quit)?;

    app.manage(TrayItems { layouts: items, autostart });

    let mut builder = TrayIconBuilder::with_id("main")
        .tooltip("Music Widget")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(on_menu_event)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                toggle_visibility(tray.app_handle());
            }
        });
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app)?;
    Ok(())
}

fn on_menu_event(app: &AppHandle, event: MenuEvent) {
    let id = event.id().as_ref();
    if let Some(layout) = Layout::ALL.into_iter().find(|&l| layout_id(l) == id) {
        layout::apply(app, layout);
        return;
    }
    match id {
        TOGGLE => toggle_visibility(app),
        AUTOSTART => {
            let autolaunch = app.autolaunch();
            let result = if autolaunch.is_enabled().unwrap_or(false) {
                autolaunch.disable()
            } else {
                autolaunch.enable()
            };
            if let Err(e) = result {
                eprintln!("failed to toggle autostart: {e}");
            }
            // Reflect the real state: the check flips on click even if the change failed.
            if let Some(items) = app.try_state::<TrayItems>() {
                let _ = items.autostart.set_checked(autolaunch.is_enabled().unwrap_or(false));
            }
        }
        QUIT => app.exit(0),
        _ => {}
    }
}

fn toggle_visibility(app: &AppHandle) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    if window.is_visible().unwrap_or(false) {
        let _ = window.hide();
    } else {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

/// Moves the radio checkmark to `layout`.
pub fn sync_layout(app: &AppHandle, layout: Layout) {
    if let Some(items) = app.try_state::<TrayItems>() {
        for (l, item) in &items.layouts {
            let _ = item.set_checked(*l == layout);
        }
    }
}
