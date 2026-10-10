# 10 - Polish: position, hotkeys, autostart

**Goal:** make it nice to live with. Tick sub-items individually; none is required for a working app.

## Checklist
- [ ] **Remember position:** save the window position on move/exit, restore on start (`tauri-plugin-window-state` does this).
- [ ] **Global media hotkeys:** `tauri-plugin-global-shortcut` for play/pause, next, prev.
- [ ] **Autostart:** `tauri-plugin-autostart`, with a tray toggle.
- [ ] **Compact mode:** a smaller layout (e.g. art + play/pause only), toggled from the tray or a double-click.
- [ ] **Later / optional:** search and playlists.

## You're done when
Restarting the app puts the widget back where you left it, and the hotkeys work while another app has focus.

## If something fails
- Hotkey does nothing: another app may already own it; try a different key combination.
- Position restores off-screen after a monitor change: clamp to a visible monitor on load.
