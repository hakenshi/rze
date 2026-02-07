# Templates (Implementer Notes)

This is the template system spec for `rze`. It must be documented better than pywal.

## Template language (v0)

- Replace `{{ ... }}` expressions.
- Expressions support variables and method chaining.

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

- Defaults live in `/usr/share/rze/templates`.
- User templates live in `~/.config/rze/templates`.
- Render outputs live in `$RZE_CACHE/out/<target>/...`.
- Deployed symlinks live under `~/.config/<app>/rze/...`.

## Cookbook

See the default packs in `assets/templates/`.
