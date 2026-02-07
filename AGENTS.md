# Agent Notes for This Repo (rze)

This file is for agentic coding tools working in `/home/yoru/Documentos/projects/rze`.
It captures how to build/lint/test and the house style / invariants.

## Project Shape

- Language: Rust (edition `2024`).
- Workspace: root crate `rze` + member crates under `crates/`.
- Domain split (see `docs/architecture.md`):
  - `crates/rze-core`: pure domain logic + value objects. No OS access.
  - `crates/rze-app`: use-cases/pipelines + port traits + explicit factories.
  - `crates/rze-infra`: adapters at OS boundaries (fs/process/ffmpeg/DE backends).
  - `src/main.rs`: clap CLI + wiring + output formatting.

Key specs/invariants live in `docs/` (start with `docs/vision.md`).

## Commands

This repo provides a `justfile` (recommended). If you have `just` installed:

- List tasks: `just`
- Build/test: `just build`, `just test`
- Format/lint: `just fmt`, `just clippy`
- Run: `just run IMG=/path/or/url`

If you do not use `just`, use Cargo directly:

- Build: `cargo build`
- Check: `cargo check --workspace`
- Format: `cargo fmt --all`
- Clippy: `cargo clippy --workspace --all-targets -- -D warnings`
- Tests: `cargo test --workspace`

## Invariants

- Cache is the source of truth: `$XDG_CACHE_HOME/rze` (fallback `~/.cache/rze`).
- Never overwrite main configs; only manage `~/.config/<app>/rze/...` and `~/.config/gtk-{3,4}.0/rze.css`.
- COSMIC wallpaper backend is default on COSMIC; generic Wayland uses `nayu`.
- Default error output is one-line; verbose only with `RZE_DEBUG=1`.
