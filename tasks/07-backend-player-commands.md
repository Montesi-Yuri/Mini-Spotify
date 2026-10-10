# 07 - Backend: player and controls

**Goal:** the app registers as a Spotify Connect device and can be controlled from the UI.

## Checklist
- [ ] Create a `Player` with the rodio audio backend from the session (follow the `examples/` for your pinned version).
- [ ] Create the `Spirc` (Connect controller) so the device appears as "MiniWidget" in Spotify.
- [ ] Keep the `Spirc` handle in Tauri state.
- [ ] Add Tauri commands, each calling the matching `Spirc` method:
  - `play_pause`
  - `next`
  - `prev`
  - `seek`
  - `set_volume`
- [ ] Register them in `invoke_handler` and test from the browser devtools: `await window.__TAURI__.core.invoke('play_pause')` (needs `withGlobalTauri` or use an imported `invoke`).

## You're done when
The device shows in Spotify, you can start a track there, and your commands pause, skip and change volume.

## Tip
If you want the playback/state logic covered by tests, write the tests first for the pure logic (e.g. progress calculation), and keep the librespot calls thin.
