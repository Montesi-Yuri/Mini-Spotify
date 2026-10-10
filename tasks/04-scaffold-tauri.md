# 04 - Scaffold the Tauri project

**Goal:** get an empty Tauri app running in this folder.

## Checklist
- [ ] From `c:\projects\music_widget` run:
  ```
  npm create tauri-app@latest
  ```
  Answer the prompts: language TypeScript/JavaScript, and a frontend template (vanilla is the lightest; Svelte or Solid are fine if you prefer them). Choose to create it in the current folder (`.`) so `tasks/` stays alongside it.
- [ ] `npm install`
- [ ] `npm run tauri dev`. The first build compiles lots of Rust and takes several minutes.
- [ ] (Optional) `git init` so you can commit your progress.

## You're done when
A default Tauri window opens.

## If something fails
- Build errors mentioning `link.exe` → task 01 (C++ workload).
- Blank white window → check the dev server URL in `tauri.conf.json` and that Vite is running.
