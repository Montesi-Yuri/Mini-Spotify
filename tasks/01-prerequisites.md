# 01 - Check your toolchain

**Goal:** make sure your PC can build a Tauri app.

## Checklist
- [ ] **Rust (MSVC):** in PowerShell run `rustc --version` and `rustup show`. The default host should be `x86_64-pc-windows-msvc`. If missing, install from https://rustup.rs.
- [ ] **VS Build Tools:** install "Visual Studio Build Tools" and tick the **Desktop development with C++** workload (MSVC + Windows SDK).
- [ ] **Node.js:** `node --version` and `npm --version`. Install the LTS release if missing.
- [ ] **WebView2:** already on Windows 11, nothing to do.

## You're done when
All four commands/installs work. Write the versions in the board's notes log.

## If something fails
- `link.exe not found` later on means the C++ workload isn't installed or you need to reopen the terminal.
- Tauri's own prerequisites page lists the same requirements for Windows.
