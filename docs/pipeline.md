# Pipeline (Implementer Notes)

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

## `rze apply [--no-reset]`

- Load `$RZE_CACHE/state.json`.
- Set wallpaper for current session using the same backend selection rules.
- Re-render templates and redeploy symlinks (always).
- Run resets unless `--no-reset`.

## Atomicity

- State file: temp + rename.
- Render outputs: temp + rename.
- cosmic-config writes: temp + rename.
- Symlink updates should be done using an atomic swap strategy.
