# Coding Style (Implementer Notes)

- Prefer small, single-purpose modules.
- Avoid hidden global state and static registries.
- Use explicit wiring and explicit factories.

## API design rules

- `rze-core` functions should be deterministic and testable.
- `rze-app` functions should read like a recipe.
- `rze-infra` should hide OS mess behind small functions.

## Error rules

- Use `anyhow` to attach context per step.
- Default CLI output is one-line.
- Debug mode (`RZE_DEBUG=1`) prints full chain with stderr snippets.

## Logging rules

- No output on success unless explicitly requested.
- Under debug, log each pipeline step start/end.
