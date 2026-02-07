# Architecture (Implementer Notes)

This project uses a lightweight clean-architecture/DDD split:

- `rze-core`: pure domain logic and value objects. No OS access.
- `rze-app`: use-cases (pipelines) + port traits + explicit factories.
- `rze-infra`: adapters for OS boundaries (ffmpeg, cosmic-config, filesystem, process runner, resets).
- root crate (`rze`): clap CLI + wiring + output formatting.

## Dependency rules

- `rze-core` must not depend on filesystem, processes, DE detection, or IPC.
- `rze-app` depends on `rze-core` and traits only.
- `rze-infra` implements the traits and may use OS features.
- CLI wires everything together.

## Port traits (high-level)

We keep traits only at true variability points:

- Wallpaper backend (COSMIC-native vs nayu vs later GNOME/KDE/X11)
- Input resolver (local path vs URL)
- Pixel decoder (ffmpeg)
- State store (atomic write)
- Template service (render -> cache -> deploy symlinks)
- Reset actions (cava/dunst/waybar/kitty/ghostty)

## Wallpaper backend selection

Selection is explicit and primarily driven by `XDG_CURRENT_DESKTOP`:

- COSMIC => cosmic-config backend (force same-on-all)
- else Wayland => `nayu set`
- else X11 => (later) X11 backend

We allow an override flag for debugging.

## Template packs

Default templates are installed to `/usr/share/rze/templates`.

At runtime we materialize user templates into `~/.config/rze/templates` if missing/empty:

1. `$XDG_DATA_HOME/rze/templates` (optional override)
2. `/usr/share/rze/templates`

User templates always win.
