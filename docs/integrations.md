# Integrations (Implementer Notes)

## COSMIC wallpaper

COSMIC wallpapers are controlled by `cosmic-bg` watching cosmic-config.

Keys we write:

- `~/.config/cosmic/com.system76.CosmicBackground/v1/same-on-all` => `true`
- `~/.config/cosmic/com.system76.CosmicBackground/v1/all` => update `source: Path("...")` to our canonical cache wallpaper path

Writes must be atomic.

## Ghostty theme (match Ghostty docs)

- Theme lookup directory: `~/.config/ghostty/themes`
- We deploy: `~/.config/ghostty/themes/rze` (symlink)
- User config: `theme = rze`
- Reload: documented keybinding `ctrl+shift+,` (do not kill ghostty by default)

## Cava

- Write a theme under `~/.config/cava/themes/rze` (symlink)
- User config: `theme = 'rze'`
- Reload: `pkill -USR2 cava`
