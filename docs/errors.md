# Errors (Implementer Notes)

## UX

- Default: one-line error.
- Debug: full error chain with context (`RZE_DEBUG=1`).

## Implementation

- Use `anyhow` for context-chaining.
- Tag errors with a stable kind for exit codes.
