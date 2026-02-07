# Templates (Implementer Notes)

This is the template system spec for `rze`.

Primary design goal: the template system must be powerful enough to theme real apps,
but small enough that the implementation is readable and debuggable.

## Template language (v0)

- Replace `{{ ... }}` expressions.
- Expressions support variables and method chaining.

## Expression model

An expression is:

- a base value: `bg`, `c4`, `wallpaper`, etc.
- followed by zero or more calls or properties, evaluated left-to-right.

Examples:

- `{{ bg }}`
- `{{ bg.hex }}`
- `{{ c4.lighten(10).hex }}`
- `{{ bg.alpha(0.85).rgba(0.85) }}`

Notes:

- We keep syntax intentionally minimal (no loops/conditionals/includes in v0).
- All numbers are parsed as decimal floats unless stated otherwise.

### Variables

- `bg`, `fg`, `cursor`
- `c0`..`c15`
- `wallpaper`

### Methods / formats

- `.hex` (default string form)
- `.rgb`
- `.rgba(a)` where `a` is 0..1
- `.hex8(a)` where `a` is 0..1

### Transforms

- `.lighten(p)` / `.darken(p)` (0..100)
- `.sat(p)` (0..100)
- `.alpha(a)` (0..1)

## Template packs

### Sources

- Defaults live in `/usr/share/rze/templates` (and optionally `$XDG_DATA_HOME/rze/templates`).
- User templates live in `~/.config/rze/templates`.

### Output layout

- Render outputs live in `$RZE_CACHE/out/<target>/...`.
- Deployed symlinks live under `~/.config/<app>/rze/...`.
- GTK is special: `~/.config/gtk-3.0/rze.css` and `~/.config/gtk-4.0/rze.css`.

### Mapping model

We do not use an external "template map" file.
Instead, a stable mapping of *target pack name* -> output paths -> deploy symlinks is implemented
in code and documented in `docs/integrations.md`.

User templates override the template *content*, not the mapping.
This keeps the system understandable and prevents accidental writes to arbitrary paths.

## Cookbook

Default pack templates live in `assets/templates/`.

### Ghostty

We match Ghostty's recommendations exactly (keys and format).

- Deploy: `~/.config/ghostty/themes/rze` (symlink)
- User sets `theme = rze` in `~/.config/ghostty/config`
- Reload: documented keybinding `ctrl+shift+,`

### Cava

- Deploy: `~/.config/cava/themes/rze` (symlink)
- User sets `theme = 'rze'`
- Reload via signal: `pkill -USR2 cava`

### GTK

- Deploy symlinks:
  - `~/.config/gtk-3.0/rze.css`
  - `~/.config/gtk-4.0/rze.css`

User needs a one-time import in `gtk.css` (we do not edit it automatically).
