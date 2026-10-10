# 11 - Package an installer

**Goal:** a Windows installer you can run on your PC (and others, with caveats).

## Checklist
- [ ] Set the app name, identifier and icon in `tauri.conf.json` (`npx tauri icon <your-image.png>` generates icon sizes).
- [ ] Run `npm run tauri build`. The installers land in `src-tauri/target/release/bundle/` (NSIS and/or MSI).
- [ ] Install it, then check: first-run login, playback, tray, hotkeys, autostart.
- [ ] If you share it: tell people it needs Spotify Premium and uses the unofficial librespot, which Spotify's terms don't allow. Unsigned installers will also trigger a Windows SmartScreen warning.

## You're done when
The installed app works like the dev version.
