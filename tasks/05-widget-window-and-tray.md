# 05 - Frameless widget window + tray

**Goal:** make the window look and behave like a widget.

## Checklist
- [ ] In `src-tauri/tauri.conf.json`, edit the window entry (under `app.windows` in Tauri 2) to roughly:
  ```json
  {
    "width": 340,
    "height": 110,
    "decorations": false,
    "alwaysOnTop": true,
    "skipTaskbar": true,
    "transparent": true,
    "resizable": false
  }
  ```
- [ ] In your HTML, add `data-tauri-drag-region` to the element you want to drag the window by (the background container, for example).
- [ ] Give the page a transparent `body` background and a rounded, coloured container so the transparency shows.
- [ ] Add a tray icon with **Show** and **Quit** menu items (see Tauri's system tray docs; needs the `tray-icon` feature in `Cargo.toml`).

## You're done when
The widget has no title bar, floats over other windows, isn't in the taskbar, can be dragged, and can be shown or quit from the tray.

## If something fails
- Transparency shows a black/white box: make sure both the window and `html, body` backgrounds are transparent.
- Can't quit the app: the tray Quit item must call the app's exit; don't rely on closing the window.
