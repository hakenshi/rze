# Config (Implementer Notes)

Config is TOML: `$XDG_CONFIG_HOME/rze/config.toml`.

The config exists to make behavior predictable without editing code.

## Goals

- keep configuration small and comprehensible
- avoid requiring config for default usage
- allow advanced users to override backend choice and reset behavior

## Proposed v0 schema

```toml
[wallpaper]
# Preferred backend list. The first applicable backend wins.
# Supported values in early versions: "cosmic", "nayu".
backends = ["cosmic", "nayu"]

[templates]
# Optional override. If unset, use $XDG_CONFIG_HOME/rze/templates.
dir = "~/.config/rze/templates"

[resets]
enabled = true

[resets.targets]
# Optional per-target toggles.
cava = true
dunst = true
waybar = true
kitty = true
ghostty = true
gtk = true
hypr = true

[downloads]
# Used by URL input and later wallhaven.
max_mb = 50
```

## Template default materialization

If the user template directory is missing or empty, we copy defaults from:

1. `$XDG_DATA_HOME/rze/templates`
2. `/usr/share/rze/templates`

Copy behavior is merge-only unless `rze init --force`.
