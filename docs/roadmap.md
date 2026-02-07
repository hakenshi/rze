# Roadmap (Implementer Notes)

## v0

- COSMIC wallpaper backend
- nayu integration for generic Wayland compositors
- palette generation (ffmpeg + deterministic quantizer)
- templates + symlink deploy
- resets + `--no-reset`
- `apply` use-case

## v0 expectations

- `rze img` works on COSMIC and generic Wayland compositors.
- COSMIC wallpaper is set via cosmic-config (native) and forces same-on-all.
- Generic Wayland wallpaper is set via `nayu set`.
- Template packs render and deploy via symlinks; never overwrite main configs.
- Resets are best-effort and can be disabled.

## v0.x

- URL input (curl download + ffmpeg verify)

## later

- wallhaven command (interactive; prints cached path)
- KDE/GNOME adapters

## Later: interactive wallpaper selection

- `rze wallhaven "<query>"` returns a cached local path by default.
- Optional: terminal previews for kitty/ghostty.
