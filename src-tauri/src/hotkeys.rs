use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::play_connect::{self, ConnectState};

type Action = fn(tauri::State<'_, ConnectState>) -> Result<(), String>;

/// Media keys first; the Ctrl+Alt combos are a fallback for keyboards without them
/// or when another app owns the media keys.
const BINDINGS: [(&str, Action); 6] = [
    ("MediaPlayPause", play_connect::play_pause),
    ("MediaTrackNext", play_connect::next),
    ("MediaTrackPrevious", play_connect::prev),
    ("Ctrl+Alt+Space", play_connect::play_pause),
    ("Ctrl+Alt+Right", play_connect::next),
    ("Ctrl+Alt+Left", play_connect::prev),
];

/// A binding that fails to register (usually because another app owns it) is
/// logged and skipped; the rest still work.
pub fn init(app: &AppHandle) {
    for (keys, action) in BINDINGS {
        let result = app
            .global_shortcut()
            .on_shortcut(keys, move |app, _shortcut, event| {
                if event.state() != ShortcutState::Pressed {
                    return;
                }
                if let Err(e) = action(app.state::<ConnectState>()) {
                    eprintln!("hotkey {keys}: {e}");
                }
            });
        if let Err(e) = result {
            eprintln!("could not register hotkey {keys}: {e}");
        }
    }
}
