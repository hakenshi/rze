# Vision (Implementer Notes)

## Goal

`rze` is a CLI that makes the system match an image:

- set wallpaper (DE/WM-aware)
- generate a palette from the image ("pywal-ish" vibe, deterministic)
- render template packs (config fragments) from the palette
- deploy fragments via symlinks
- reload/restart apps (default on; `--no-reset` opt-out)

`rze img <input>` is the primary user command.

## Invariants (non-negotiable)

- **Cache is the source of truth.** Generated artifacts live in `$XDG_CACHE_HOME/rze` (fallback `~/.cache/rze`).
- **Never overwrite main configs.** Only create/update `~/.config/<app>/rze/...` files (symlinks) and `~/.config/gtk-{3,4}.0/rze.css`.
- **COSMIC wallpaper is native by default.** If COSMIC is detected, set wallpaper by updating cosmic-config keys (force same-on-all). Do not use `nayu` unless forced.
- **Generic Wayland compositors use nayu.** When no native DE backend exists and we are on Wayland, use `nayu set <path>`.
- **Deterministic palette generation.** Same input + same settings => same output palette.
- **Error UX is stable.** One-line error by default; verbose chain only with `RZE_DEBUG=1`.
- **Resets are stable.** Resets/reloads are enabled by default; `--no-reset` disables them.
- **`rze apply` always re-renders.** This guarantees template edits take effect.

## Non-goals (v0 scope boundaries)

- No pywal template compatibility.
- No fancy wallpaper transitions.
- GNOME/KDE wallpaper/theme adapters are planned but not required for initial COSMIC + generic Wayland support.
