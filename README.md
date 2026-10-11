# Music Widget

A small always-on-top desktop widget that shows and controls Spotify playback. Built with Tauri 2 (Rust) and React + TypeScript.

The app registers itself as a Spotify Connect device using [librespot](https://github.com/librespot-org/librespot), so it plays audio itself and also appears in Spotify's device list.

## Features

- Borderless, transparent, always-on-top widget with cover art, track info, playback controls, progress bar and volume.
- Three layouts, chosen by the widget's width (the height is fixed):
  - **Full**: everything.
  - **Compact** (up to 480px wide): cover art, track text and controls.
  - **Extra-compact** (up to 160px wide): cover art with a single play/pause button.
- Free horizontal resizing, double-click to cycle through the layout presets.
- Edge and corner docking, plus a "fit width" action that stretches the widget across its monitor.
- System tray menu: show/hide, layout presets, fit width, start at login, quit.
- Global hotkeys that work while another app has focus.
- Position and size are remembered between runs and kept on a visible monitor.
- Optional start at login (off until you turn it on in the tray).

### Global hotkeys

| Action     | Keys                            |
| ---------- | ------------------------------- |
| Play/pause | `MediaPlayPause`, `Ctrl+Alt+Space` |
| Next       | `MediaTrackNext`, `Ctrl+Alt+Right` |
| Previous   | `MediaTrackPrevious`, `Ctrl+Alt+Left` |

If another app already owns a key, that binding is skipped and the rest still work.

## Requirements

- A **Spotify Premium** account (required for librespot playback).
- [Node.js](https://nodejs.org/) and npm.
- [Rust](https://www.rust-lang.org/tools/install) (2024 edition toolchain).
- The [Tauri 2 prerequisites](https://tauri.app/start/prerequisites/) for your OS (on Windows: MSVC build tools and WebView2).

## Getting started

```bash
npm install
npm run tauri dev
```

On first launch a browser window opens for Spotify login (OAuth callback on `http://127.0.0.1:8898/login`). Credentials are cached in the app's data directory, so later launches don't need the browser.

## Scripts

| Command             | What it does                              |
| ------------------- | ----------------------------------------- |
| `npm run dev`       | Vite dev server only (frontend)           |
| `npm run build`     | Type-check and build the frontend         |
| `npm run tauri dev` | Run the full app in development           |
| `npm run tauri build` | Produce a release build and installers  |

## Project structure

```
src/                  React frontend
  Player.tsx          Widget UI and layouts (Player.css holds the width breakpoints)
  hooks/player.ts     Player state and commands
  definitions/        Shared types
src-tauri/src/
  lib.rs              App setup
  auth.rs             Spotify OAuth and credential cache
  play_connect.rs     librespot Connect device and playback commands
  player_event.rs     Track/playback events sent to the frontend
  layout.rs           Layouts, presets, docking, fit width
  tray.rs             System tray menu
  hotkeys.rs          Global hotkeys
CONTEXT.md            Glossary of domain terms used in the code
```

The layout width thresholds live in both `layout.rs` and `Player.css`; keep them in sync.

## Recommended IDE setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
