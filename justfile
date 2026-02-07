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

cov:
  cargo llvm-cov --workspace --fail-under-lines 80

fmt:
  cargo fmt --all

clippy:
  cargo clippy --workspace --all-targets -- -D warnings
