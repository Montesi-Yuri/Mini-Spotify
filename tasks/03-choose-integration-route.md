# 03 - Decide: sidecar (A) or embedded (B)

**Goal:** pick how the app talks to librespot. Decide before starting task 06.

## The two routes

| | A. Sidecar | B. Embedded |
|---|---|---|
| How | Your app launches the librespot executable and controls it through the Spotify Web API | Your Rust code uses the librespot crates directly |
| Track info | `--onevent` hook or polling | `PlayerEvent`s |
| Control | Web API calls | `Spirc` |
| Extra setup | Spotify developer app + OAuth | None |
| Risk | More moving parts, API rate limits | Crate API is unstable between versions |

**Recommendation: B.** Fewer moving parts and no developer app. Pin exact crate versions and read the crates' `examples/` folder. If the crate API fights you, fall back to A.

## Checklist
- [ ] Choose A or B.
- [ ] Record the choice in the board's notes log.
- [ ] If B: look at the librespot repo's `examples/` and note which version/tag you'll pin.

## You're done when
The decision is written down.
