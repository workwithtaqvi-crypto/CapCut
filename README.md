# LiteCut

An ultra-lightweight desktop video editor written in Rust, designed to run smoothly on
low-end hardware (4 GB RAM / 1 GB GPU).

## Status: Phase 1 - Home dashboard

- Dark theme home page: profile card, Home / Share history, Spaces, "Create project"
  banner, "Long video to shorts", Projects section (search, view switch, Trash),
  empty state, and bottom-left promo cards.
- Immediate-mode UI (egui + OpenGL), no web view, no background threads at idle.
- The editor itself, timeline and video decoding come in later phases.

## Download (Windows, portable)

Every push to `main` builds `litecut.exe` automatically with GitHub Actions.

- Latest build: open the **Releases** page of this repo and download `litecut.exe`
- Or: **Actions** tab -> latest run -> **Artifacts** -> `litecut-windows-portable`

No installer needed - just double-click the exe.

## Build locally

```
cargo run --release
```

## Roadmap

1. Home page (this phase)
2. Editor shell: media pool, preview, timeline layout
3. Hardware-accelerated decode + 2K playback
4. Export / render
