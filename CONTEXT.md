# Music Widget

A small always-on-top desktop widget that shows and controls Spotify playback.

## Language

**Widget**:
The borderless, always-on-top window showing the current track and playback controls.

**Layout**:
How the Widget arranges its content, determined by the Widget's current width: Full, Compact or Extra-compact. Not a stored choice. Distinct from the player's playing/paused state.
_Avoid_: mode, view, size

**Full layout**:
Cover art, track text, playback controls, progress bar and volume.

**Compact layout**:
Cover art on the left; track text above the playback controls. No progress bar or volume.

**Extra-compact layout**:
Cover art filling the Widget with a single play/pause button on it. Nothing else.

**Preset**:
A Layout's standard size. Choosing a Layout in the Tray, or double-clicking the Widget, resizes the Widget to a Preset; the user can also resize it freely, and the width picks the Layout.

**Docking**:
The Widget snapping flush to a screen edge or corner when dropped close to it.

**Fit width**:
Stretching the Widget across the full width of its monitor.

**Tray**:
The system tray icon and its menu. Shows or hides the Widget, applies a Layout Preset, fits the width, toggles Start at login, and quits.

**Start at login**:
Whether the app launches automatically when the user signs in to the OS. Off until the user opts in.

**Global hotkey**:
A keyboard shortcut for play/pause, next or previous that works while another application has focus.

## Relationships

- The Widget has one Layout at any time, set by its width. Its height is fixed.
- The Widget's position and size persist across restarts and are kept on a visible monitor.
- The Tray and a double-click on the Widget apply Presets; double-click cycles Full → Compact → Extra-compact → Full.
- Global hotkeys act on playback regardless of the current Layout.
