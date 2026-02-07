set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default:
  @just --list

build:
  cargo build

run IMG:
  cargo run -- img "{{IMG}}"

apply:
  cargo run -- apply

init:
  cargo run -- init

test:
  cargo test --workspace

fmt:
  cargo fmt --all

clippy:
  cargo clippy --workspace --all-targets -- -D warnings
