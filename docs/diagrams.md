# Diagrams (Implementer Notes)

## Layers

```text
CLI -> app -> core
        ^
        |
      infra
```

## `rze img` (sequence)

```text
rze img <input>
  -> ensure templates
  -> resolve input (path or URL)
  -> persist cache wallpaper
  -> set wallpaper (COSMIC native OR nayu)
  -> decode pixels (ffmpeg)
  -> quantize + roles
  -> write state.json
  -> render templates to cache
  -> deploy symlinks
  -> run resets (unless --no-reset)
```

## Wallpaper backend decision tree

```text
if XDG_CURRENT_DESKTOP contains COSMIC:
  use cosmic-config
else if WAYLAND_DISPLAY is set:
  call "nayu set"
else if DISPLAY is set:
  X11 backend (later)
else:
  error
```

## Template deployment

```text
template packs (user overrides) -> render -> $RZE_CACHE/out/<target>/...
  -> symlink -> ~/.config/<app>/rze/...
```
