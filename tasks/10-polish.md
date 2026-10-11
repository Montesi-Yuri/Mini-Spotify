# 10 - Polish: position, hotkeys, autostart

**Goal:** make it nice to live with. Tick sub-items individually; none is required for a working app.

## Checklist
- [x] **Remember position:** save the window position on move/exit, restore on start (`tauri-plugin-window-state` does this). Position only, clamped to a visible monitor on start.
- [x] **Global media hotkeys:** `tauri-plugin-global-shortcut` for play/pause, next, prev.
- [x] **Autostart:** `tauri-plugin-autostart`, with a tray toggle.
- [x] **Compact mode:** Full / Compact / Extra-compact (art + play/pause only) layouts, chosen from the tray or cycled by double-click. Persisted in `settings.json`.
- [ ] **Later / optional:** search and playlists. Deferred to their own task.

## You're done when
Restarting the app puts the widget back where you left it, and the hotkeys work while another app has focus.

## If something fails
- Hotkey does nothing: another app may already own it; try a different key combination.
- Position restores off-screen after a monitor change: clamp to a visible monitor on load.
