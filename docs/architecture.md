# Architecture (Implementer Notes)

This document explains how `rze` is structured so the code stays plug-and-play:
change one subsystem without breaking the rest.

## Crates

- `rze-core`
  - Pure domain logic and value objects.
  - Must not touch filesystem, processes, environment variables, or IPC.
  - Owns: `Color`, `Palette16`, `Roles`, quantization, role assignment, template engine.

- `rze-app`
  - Use-cases/pipelines and port traits.
  - Must not do OS work directly.
  - Owns: `ImgUseCase`, `ApplyUseCase`, `InitUseCase`, `EnvUseCase`.
  - Owns: explicit factories that select implementations based on environment/config.

- `rze-infra`
  - Adapters for OS boundaries.
  - Owns: process runner, ffmpeg decoder, curl downloader, atomic writes, symlinks, cosmic-config edits, reset actions.

- root crate (`rze`)
  - CLI parsing (clap), wiring (construct use-cases from infra), output formatting.
  - One-line error by default; verbose with `RZE_DEBUG=1`.

## Dependency rules

- `rze-core` depends on std only.
- `rze-app` depends on `rze-core` and traits (ports). It may depend on `anyhow` for error context.
- `rze-infra` depends on `rze-core` and implements `rze-app` ports.
- root crate wires everything together.

Rule of thumb: all OS behavior should be reachable by searching for a port impl in `rze-infra`.

## Ports (traits)

Ports exist only at variability points.

- `WallpaperBackend`
  - `set_wallpaper(path)` sets wallpaper for the current session.
  - Implementations:
    - COSMIC backend (native cosmic-config edits)
    - nayu backend (calls `nayu set`)
    - GNOME/KDE/X11 backends later

- `InputResolver`
  - resolves `path-or-url` into a verified local file path.
  - local input: must exist and be ffmpeg-decodable.
  - url input: download -> verify by decoding.

- `PixelDecoder`
  - decodes pixels from a local image using ffmpeg (scaled down for palette generation).

- `StateStore`
  - reads/writes `$RZE_CACHE/state.json` atomically.

- `TemplateService`
  - renders template packs into `$RZE_CACHE/out/<target>/...`.
  - deploys symlinks into `~/.config/...`.

- `ResetAction` / `ResetRunner`
  - reload/restart external programs after theme updates.
  - gated by `--no-reset`.

## Wallpaper backend selection

Selection is explicit, driven primarily by `XDG_CURRENT_DESKTOP`.

Default order (configurable):

1. COSMIC -> COSMIC backend (native, forces same-on-all)
2. otherwise, if Wayland -> `nayu set`
3. otherwise, if X11 -> (later) X11 backend (or use nayu-x11)

We provide a CLI override flag for debugging and for users that intentionally want non-default behavior.

## Template packs

### Sources

Default packs are installed to `/usr/share/rze/templates`.

At runtime, `rze` materializes user templates into `~/.config/rze/templates` if missing/empty.
The source search order is:

1. `$XDG_DATA_HOME/rze/templates` (optional override)
2. `/usr/share/rze/templates`

User templates always win.

### Outputs

Templates are rendered into `$RZE_CACHE/out/<target>/...`.
Deployed symlinks are created in app-specific `rze` namespaces (never overwrite main configs).

The mapping of target -> output path -> deploy path is a stable contract and is documented in `docs/integrations.md`.
