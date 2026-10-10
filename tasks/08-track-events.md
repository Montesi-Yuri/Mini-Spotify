# 08 - Backend: send track info to the UI

**Goal:** the frontend learns what's playing, in real time.

## Checklist
- [ ] Subscribe to the player's event channel and handle track change, play, pause, seek and volume events.
- [ ] On track change, fetch metadata with `librespot-metadata` (title, artist, cover image id → image URL, duration).
- [ ] Emit a Tauri event named `track` with a payload like:
  ```json
  { "title": "", "artist": "", "artUrl": "", "durationMs": 0, "positionMs": 0, "playing": false }
  ```
- [ ] Also emit on play/pause/seek so the position stays correct.
- [ ] Verify by listening in the frontend: `listen('track', e => console.log(e.payload))`.

## You're done when
Changing songs or pausing from the Spotify app logs a correct payload in the devtools console.

## If something fails
- No cover art: check how your version maps the image id to a URL (usually `https://i.scdn.co/image/<id>`).
- Position drifts: send `positionMs` plus a timestamp, and let the frontend extrapolate.
