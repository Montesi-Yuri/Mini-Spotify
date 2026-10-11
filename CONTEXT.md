# Music Widget

A small always-on-top desktop widget that shows and controls Spotify playback.

## Language

**Widget**:
The borderless, always-on-top window showing the current track and playback controls.

**Layout**:
How the Widget arranges its content. One of three: Full, Compact, Extra-compact. Distinct from the player's playing/paused state.
_Avoid_: mode, view, size

**Full layout**:
Cover art, track text, playback controls, progress bar and volume.

**Compact layout**:
Cover art on the left; track text above the playback controls. No progress bar or volume.

**Extra-compact layout**:
Cover art filling the Widget with a single play/pause button on it. Nothing else.

**Tray**:
The system tray icon and its menu. Shows or hides the Widget, selects the Layout, toggles Start at login, and quits.

**Start at login**:
Whether the app launches automatically when the user signs in to the OS. Off until the user opts in.

**Global hotkey**:
A keyboard shortcut for play/pause, next or previous that works while another application has focus.

## Relationships

- The Widget has exactly one Layout at a time; the chosen Layout and the Widget's position persist across restarts.
- The Tray and a double-click on the Widget both change the Layout; double-click cycles Full → Compact → Extra-compact → Full.
- Global hotkeys act on playback regardless of the current Layout.
