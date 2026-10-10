# 09 - Frontend: the player UI

**Goal:** a compact player that shows what's playing and controls it.

## Checklist
- [ ] Layout (~340x110): cover art on the left; title, artist, progress bar and buttons on the right.
- [ ] Buttons: previous, play/pause, next, calling `invoke('prev' | 'play_pause' | 'next')`.
- [ ] Progress bar: clicking it calls `invoke('seek', ...)`.
- [ ] Listen to the `track` event and update the display.
- [ ] Run progress locally with a timer (e.g. every 250-500 ms while playing) and reset it from each `track` event.
- [ ] Truncate long titles with an ellipsis; make sure the drag region still works.
- [ ] (Optional) sketch the layout in a quick throwaway page first before committing to it.

## You're done when
The widget mirrors playback accurately and every button works.
