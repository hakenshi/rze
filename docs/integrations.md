# Integrations (Implementer Notes)

This document defines stable integration contracts:

- where outputs live in cache
- where we deploy symlinks
- what the user must include once (if anything)
- what reset action is used

All paths here are part of the intended user-facing contract.

## COSMIC wallpaper

COSMIC wallpapers are controlled by `cosmic-bg` watching cosmic-config.

Keys we write:

- `~/.config/cosmic/com.system76.CosmicBackground/v1/same-on-all` => `true`
- `~/.config/cosmic/com.system76.CosmicBackground/v1/all` => update `source: Path("...")` to our canonical cache wallpaper path

Writes must be atomic.

### Behavior

- Force "same wallpaper on all displays".
- Preserve other COSMIC wallpaper settings; only update the `source: Path("...")`.

## Ghostty theme (match Ghostty docs)

- Theme lookup directory: `~/.config/ghostty/themes`
- We deploy: `~/.config/ghostty/themes/rze` (symlink)
- User config: `theme = rze`
- Reload: documented keybinding `ctrl+shift+,` (do not kill ghostty by default)

### Mapping

- Render output (cache): `$RZE_CACHE/out/ghostty/rze`
- Deploy symlink: `~/.config/ghostty/themes/rze -> $RZE_CACHE/out/ghostty/rze`

## Cava

- Write a theme under `~/.config/cava/themes/rze` (symlink)
- User config: `theme = 'rze'`
- Reload: `pkill -USR2 cava`

### Mapping

- Render output (cache): `$RZE_CACHE/out/cava/rze`
- Deploy symlink: `~/.config/cava/themes/rze -> $RZE_CACHE/out/cava/rze`

## Kitty

### Mapping

- Render output (cache): `$RZE_CACHE/out/kitty/colors.conf`
- Deploy symlink: `~/.config/kitty/rze/colors.conf -> $RZE_CACHE/out/kitty/colors.conf`

### One-time user include

In `~/.config/kitty/kitty.conf`:

- `include rze/colors.conf`

## Dunst

### Mapping

- Render output (cache): `$RZE_CACHE/out/dunst/colors.conf`
- Deploy symlink: `~/.config/dunst/rze/colors.conf -> $RZE_CACHE/out/dunst/colors.conf`

### One-time user include

User must include the fragment in their `dunstrc`.
We do not auto-edit it.

## GTK

### Mapping

- Render outputs (cache):
  - `$RZE_CACHE/out/gtk/gtk-3.0.css`
  - `$RZE_CACHE/out/gtk/gtk-4.0.css`
- Deploy symlinks:
  - `~/.config/gtk-3.0/rze.css -> $RZE_CACHE/out/gtk/gtk-3.0.css`
  - `~/.config/gtk-4.0/rze.css -> $RZE_CACHE/out/gtk/gtk-4.0.css`

### One-time user include

User adds `@import url("rze.css");` to:

- `~/.config/gtk-3.0/gtk.css`
- `~/.config/gtk-4.0/gtk.css`

## Waybar

### Mapping

- Render output (cache): `$RZE_CACHE/out/waybar/colors.css`
- Deploy symlink: `~/.config/waybar/rze/colors.css -> $RZE_CACHE/out/waybar/colors.css`

### One-time user include

User adds `@import "rze/colors.css";` to their Waybar stylesheet.

## Hyprland

### Mapping

- Render output (cache): `$RZE_CACHE/out/hypr/colors.conf`
- Deploy symlink: `~/.config/hypr/rze/colors.conf -> $RZE_CACHE/out/hypr/colors.conf`

### One-time user include

User adds `source = ~/.config/hypr/rze/colors.conf` to `hyprland.conf`.
