# Board: compact Spotify-like Windows widget

**Goal:** a small always-on-top Windows 11 widget to control and play Spotify, lighter than the desktop client.
**Stack:** Rust + Tauri (WebView2), playback through librespot.
**Heads up:** needs Spotify Premium. librespot is unofficial and against Spotify's ToS, and can break when Spotify changes things. Keep that in mind before sharing the app.

## How to use this board
Edit the Status column by hand: change `[ ]` to `[x]` when a task is done. Each task file has its own checklist; tick the board only when the whole file is done.

## Board

| Done | # | Task | Needs | Est. |
|:----:|---|------|-------|------|
| [x] | 01 | [Check your toolchain](01-prerequisites.md) | - | 30 min |
| [x] | 02 | [Verify librespot plays audio](02-verify-librespot.md) | 01 | 20 min |
| [x] | 03 | [Decide: sidecar (A) or embedded (B)](03-choose-integration-route.md) | 02 | 15 min |
| [x] | 04 | [Scaffold the Tauri project](04-scaffold-tauri.md) | 01 | 20 min |
| [x] | 05 | [Frameless widget window + tray](05-widget-window-and-tray.md) | 04 | 1 h |
| [ ] | 06 | [Backend: login and session](06-backend-auth-session.md) | 03, 05 | 2-3 h |
| [ ] | 07 | [Backend: player and controls](07-backend-player-commands.md) | 06 | 2-3 h |
| [ ] | 08 | [Backend: send track info to the UI](08-track-events.md) | 07 | 1-2 h |
| [ ] | 09 | [Frontend: the player UI](09-frontend-ui.md) | 05, 08 | 2-3 h |
| [ ] | 10 | [Polish: position, hotkeys, autostart](10-polish.md) | 09 | 2-3 h |
| [ ] | 11 | [Package an installer](11-packaging.md) | 10 | 1 h |

Estimates are rough guesses, not measured.

## Suggested order
01 → 02 → 03 can be done in parallel with 04 → 05 (window shell). Then 06 → 11 in order.

## Notes log
Use this space for versions, decisions and problems you hit.

- librespot version: _(fill in during 02)_
- Integration route: _(fill in during 03)_
