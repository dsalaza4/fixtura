# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.3] - 2026-05-09

### Added
- MSRV badge in README
- GitHub issue templates (bug report, feature request)
- Crate-level documentation on docs.rs with quick start, attribute overview, and examples

### Changed
- Edition bumped from 2021 to 2024 (matches MSRV 1.85)

## [0.8.2] - 2026-05-09

### Added
- MSRV declared as Rust 1.85 (Rust 2024 edition) in both crate manifests
- MSRV CI job verifying `cargo check` on Rust 1.85

## [0.8.1] - 2026-05-09

### Fixed
- Corrected `repository` URL in `fixtura` crate metadata (was `dsalazar4`, now `dsalaza4`)

## [0.8.0] - 2026-05-09

### Added
- `serde` + `Dummy` + `#[fixtura(...)]` combination explicitly tested
- Plain injection tested for tuple structs and unit structs

### Changed
- README: attribute ordering for async tests explicitly documented
- README: field override limitation for non-named structs noted
- CHANGELOG added

## [0.7.2] - 2026-05-09

### Changed
- README: restructured with features section, GitHub admonitions, and chained references example
- README: features section uses bold labels and consistent abstraction level

## [0.7.1] - 2026-05-09

### Changed
- README rewritten for clarity and concision with before/after opening

## [0.7.0] - 2026-05-09

### Changed
- Multiple `#[fixtura]` errors on the same function now all reported in one compilation instead of stopping at the first
- Duplicate `#[fixtura]` attribute on the same argument detected at compile time with a clear error
- `quote_spanned!` used throughout so errors point to the argument site, not the macro call site

## [0.6.0] - 2026-05-06

### Added
- `#[fixtura]` marker for inject mode: explicitly marks which args fixtura owns vs. passes through
- Pass-through args: unmarked args in `#[fixtura::inject]` remain in the generated signature for other frameworks (e.g. `sqlx::test`) to inject
- Pass-through idents can be referenced inside `#[fixtura(...)]` override expressions

### Changed
- `#[with(...)]` renamed to `#[fixtura(...)]` for consistency with the crate name

## [0.5.0] - 2026-05-06

### Added
- Seeded RNG: every test run uses a `StdRng` seeded from a `u64`
- Auto-seed: seed is randomly generated per run and printed to stderr only on failure, so passing tests stay silent
- Manual seed: `#[fixtura::test(seed = N)]` and `#[fixtura::inject(seed = N)]` for fully deterministic tests

## [0.4.0] - 2026-05-06

### Added
- `#[fixtura::inject]` proc-macro for async test support — injects args without emitting `#[test]`
- `#[fixtura::test]` now emits a clear compile error when applied to an `async fn`

## [0.3.0] - 2026-05-06

### Added
- Forward reference detection: referencing a later arg in `#[with(...)]` is a compile error with a message pointing to the offending ident
- Self reference detection: an arg cannot reference itself in `#[with(...)]`

## [0.2.0] - 2026-05-06

### Added
- `#[with(field = expr, ...)]` syntax for field overrides
- Multiple fields can be overridden in one attribute
- Override expressions can be any valid Rust expression (literals, method calls, computed values)
- Nested field paths: `#[with(address.city = "Portland".to_string())]`
- Cross-argument references: later args can reference fields of earlier args since bindings are evaluated top-to-bottom

## [0.1.0] - 2026-05-06

### Added
- `#[fixtura::test]` proc-macro: injects `fake::Dummy` types as test arguments
- Primitive types (`u32`, `String`, `bool`, etc.) supported out of the box
- No-arg tests supported (macro emits `#[test]` and leaves the body unchanged)
- `#[should_panic]` composes correctly
