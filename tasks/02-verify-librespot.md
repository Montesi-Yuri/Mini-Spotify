# 02 - Verify librespot plays audio

**Goal:** prove librespot works on your machine before building on it.

## Checklist
- [ ] Run `librespot --version`. Note the version and how you installed it (cargo, release binary, etc.) in the board's notes log.
- [ ] Start it:
  ```
  librespot --name "MiniWidget" --backend rodio
  ```
  Use the OAuth login the installed version offers (it prints a URL to open in the browser; flags differ by version, so check `librespot --help`).
- [ ] Open the Spotify app, click "Connect to a device", and confirm **MiniWidget** is listed.
- [ ] Select it and play a track. You should hear audio.

## You're done when
The device shows up and you hear sound.

## If something fails
- **No sound:** check the Windows default output device, or try `--backend rodio --device ?` (some versions list devices).
- **Device not listed:** same Spotify account on both? Try restarting librespot; check that the firewall allows it.
- **Login errors:** an old version may use a login method Spotify has since changed. Updating librespot is the first thing to try.
