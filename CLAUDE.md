# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`simple-easing` is a small, dependency-free Rust library crate (edition 2024) of easing functions
modelled on [easings.net](https://easings.net/). Published to crates.io; upstream repo is on
GitLab (`gitlab.com/chpio/simple-easing`).

## Commands

```sh
cargo build
cargo test                          # tests/easing.rs + doctests in src/lib.rs
cargo test --test easing out_mirrors_in  # run a single integration test
cargo clippy --all-targets          # pedantic + nursery lints are enabled in src/lib.rs
cargo fmt                           # uses hard tabs (.rustfmt.toml)
```

## Architecture and conventions

- One module per easing family (`quad`, `cubic`, `quart`, `quint`, `sine`, `expo`, `circ`,
  `back`, `elastic`, `bounce`), each glob re-exported from `src/lib.rs`, so the public API is
  flat (`simple_easing::cubic_in`). `linear`, `reverse` and `roundtrip` live directly in `lib.rs`.
- Every public function has the identical signature `fn(f32) -> f32` so they can be stored as
  fn pointers — keep it that way. Input `t` is progress in `0.0..=1.0`; output is nominally
  `0.0..=1.0` but may overshoot (back, elastic).
- Naming: `<family>_in`, `<family>_out`, `<family>_in_out`, each with a doc comment linking to
  the matching easings.net anchor (e.g. `/// <https://easings.net/#easeInOutQuad>`).
- Every public function carries `#[inline]` and `#[must_use]`.
- Clippy `pedantic` and `nursery` are warned crate-wide (`missing_const_for_fn` allowed). The
  arithmetic is written with `mul_add`, `f32::midpoint`, `exp2`, etc. to satisfy these lints;
  write new code the same way and keep `cargo clippy` warning-free.
- Formatting: tabs for indentation (`.editorconfig`, `.rustfmt.toml`).
- MSRV is 1.85 (`rust-version` in `Cargo.toml`); clippy's `incompatible_msrv` flags newer APIs.

## Tests

`tests/easing.rs` checks every function against an `f64` reference implementation transcribed
from the easings.net formulas (`REFERENCES`), and checks endpoints, `out`/`in` mirroring,
`in_out` point symmetry and continuity at `0.5` per family (`FAMILIES`). A new easing function
must be added to `REFERENCES` and, if it forms an `in`/`out`/`in_out` triple, to `FAMILIES`.

## Releases and commits

- Commits follow Conventional Commits (`feat:`, `fix:`, `test:`, `docs:`, `chore:`, `style:`,
  `build:`).
- User-visible changes get an entry in `CHANGELOG.md` (under `## Unreleased` until a release);
  version bumps are a separate commit (`chore: bump version (vX.Y.Z)`).
