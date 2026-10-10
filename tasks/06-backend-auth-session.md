# 06 - Backend: login and session

**Goal:** the app logs in to Spotify and holds a librespot session. (Written for route B. For route A, this becomes "spawn the librespot process + set up Web API OAuth".)

## Checklist
- [ ] In `src-tauri/Cargo.toml`, add `librespot-core`, `librespot-oauth`, `librespot-playback`, `librespot-connect`, `librespot-metadata` and `tokio`. **Pin exact versions**, all the same release.
- [ ] Open the librespot repo's `examples/` for your pinned version; copy the login/session pattern from there. The API changes between versions, so don't rely on snippets from memory.
- [ ] Log in with the OAuth flow (opens the browser). Save the resulting credentials in the app data dir (Tauri gives you the path) so later launches skip the browser.
- [ ] On startup: try cached credentials first, fall back to OAuth.
- [ ] Create the `Session` in a tokio task and keep it in Tauri managed state.
- [ ] Print a log line when the session connects.

## You're done when
First run opens the browser and connects; second run connects without it.

## If something fails
- Compile errors after upgrading a crate: the versions of all librespot crates must match.
- Auth rejected: check you're on a recent librespot release; old login methods stop working.
- Debug tip: set `RUST_LOG=debug` and read the output.
