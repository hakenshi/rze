# Pipeline (Implementer Notes)

This file defines the observable behavior of the use-cases.
Implementation must match this ordering and failure behavior.

## `rze img <path-or-url> [--no-reset]`

Ordered steps:

1. Ensure templates exist (auto-init defaults into `~/.config/rze/templates` if missing/empty).
2. Resolve input:
   - local path: must exist and be decodable by ffmpeg
   - URL: download, verify by decoding with ffmpeg
3. Persist canonical wallpaper under cache:
   - `$RZE_CACHE/wallpaper/current.<ext>`
4. Set wallpaper:
   - COSMIC: update cosmic-config keys (atomic)
   - generic Wayland: call `nayu set <path>`
5. Generate palette:
   - ffmpeg decode (scaled) -> pixels
   - quantize deterministically -> 16 colors
   - assign roles (bg/fg/cursor) with contrast guardrails
6. Save `$RZE_CACHE/state.json` atomically.
7. Render templates to `$RZE_CACHE/out/<target>/...` atomically.
8. Deploy symlinks to `~/.config/<app>/rze/...` and GTK `rze.css`.
9. Run reset actions unless `--no-reset`.

### Files touched

- Cache root: `$XDG_CACHE_HOME/rze` (fallback `~/.cache/rze`)
- Canonical wallpaper: `$RZE_CACHE/wallpaper/current.<ext>`
- State: `$RZE_CACHE/state.json`
- Render outputs: `$RZE_CACHE/out/<target>/...`
- Symlinks under `~/.config/<app>/rze/...` plus GTK `~/.config/gtk-{3,4}.0/rze.css`.

### Failure rules

- If wallpaper setting fails, the command fails (do not silently continue).
- If palette generation fails, the command fails (templates must not render from missing palette).
- If a subset of templates fail to render:
  - fail the command (default), and do not run resets.
  - (future: allow `--best-effort`)
- Reset failures are non-fatal by default (log and continue) unless configured otherwise.

## `rze apply [--no-reset]`

- Load `$RZE_CACHE/state.json`.
- Set wallpaper for current session using the same backend selection rules.
- Re-render templates and redeploy symlinks (always).
- Run resets unless `--no-reset`.

### Why apply re-renders

`apply` is used when switching sessions (COSMIC today, Hyprland tomorrow) and when editing templates.
Re-rendering guarantees that template edits take effect without requiring a new wallpaper selection.

## Atomicity

- State file: temp + rename.
- Render outputs: temp + rename.
- cosmic-config writes: temp + rename.
- Symlink updates should be done using an atomic swap strategy.

### Symlink atomic swap strategy

To avoid partially updated configs:

1. Render into cache with temp+rename.
2. For deploy symlinks, create a new symlink next to the old one, then rename.

Do not remove the existing target before the replacement exists.
