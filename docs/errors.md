# Errors (Implementer Notes)

## UX

- Default: one-line error.
- Debug: full error chain with context (`RZE_DEBUG=1`).

## Implementation

- Use `anyhow` for context-chaining.
- Tag errors with a stable kind for exit codes.

## Error kinds (planned)

Stable kinds for scripting and for consistent user output:

- `Config`
- `Input`
- `Download`
- `Verify`
- `Wallpaper`
- `Decode`
- `Palette`
- `Template`
- `Deploy`
- `Reset`

## Exit codes (planned mapping)

- 2: usage/CLI parse
- 10: config
- 20: input/resolve
- 30: download/verify
- 40: wallpaper
- 50: decode/palette
- 60: template/render/deploy
- 70: reset

## Context guidelines

Every boundary-crossing operation must attach context:

- which step failed (e.g. "decode pixels")
- which path was involved
- which command was executed (if any)
- exit code and stderr excerpt in debug mode
