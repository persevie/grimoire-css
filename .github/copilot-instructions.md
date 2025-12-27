# Copilot instructions (grimoire-css)

## Project shape (architecture guide)

Note: This document captures the current architecture conventions for this repo.
- Rust workspace is a single crate with **bin + lib**: `src/main.rs` calls `grimoire_css_lib::start_as_cli` (keep `main.rs` thin).
- Command routing lives in `src/commands/handler.rs` (`init` / `build` / `shorten`). Add new CLI modes by wiring them here.
- Core pipeline logic lives in `src/core/` (config → parse → build → optimize → output). Keep it side-effect-light where practical.
- External integrations live in `src/infrastructure/` (e.g. LightningCSS optimizer + miette diagnostics). Don’t mix vendor glue into `src/core/`.

## Runtime modes & config conventions
- FS mode uses a single repo config file at `grimoire/config/grimoire.config.json` (created by `init`; used by `build`). See `src/core/filesystem.rs` and `src/commands/init.rs`.
- Optimizer uses `.browserslistrc` from repo root; if missing, it is created with `defaults` (see `src/infrastructure/lightning_css_optimizer.rs`).
- Parallel project builds are opt-in via `GRIMOIRE_CSS_JOBS` (project-level isolation) — see `src/core/css_builder/css_builder_fs.rs`.
- Locking: setting `lock: true` in config enables tracking + cleanup of stale generated files via `grimoire/grimoire.lock.json` (see `src/core/file_tracker.rs`).
- Config supports external scroll/variable files: `grimoire.*.scrolls.json` and `grimoire.*.variables.json` are loaded and merged during `ConfigFs::load` (see `src/core/config/config_fs.rs`).

## Error/reporting pattern
- Prefer returning `GrimoireCssError` from core logic; attach source context when you have file content/spans using `GrimoireCssError::with_source(...)` and `SourceFile` (pattern in `src/core/parser/parser_fs.rs`).
- CLI pretty-printing goes through `GrimoireCssDiagnostic` + `miette` (see `src/infrastructure/diagnostics.rs` and `src/lib.rs`).

## Developer workflows (match CI)
- Format: `cargo fmt -- --check`
- Lint: `cargo clippy -- -D warnings`
- Tests: `cargo test`
- Coverage (CI uses this): `./scripts/coverage.sh` (requires `grcov` + `llvm-tools-preview`).

## Local running tips
- CLI (debug): `cargo run -- build` / `cargo run -- init` / `cargo run -- shorten`
- Release binary: `cargo build --release` → `target/release/grimoire_css`
- Benchmark harness expects the release binary at `../target/release/grimoire_css` (see `benchmark/README.md`).

## Versioning convention
- PR branches `rc/x.y.z` must match `version = "x.y.z"` in `Cargo.toml` (enforced by `.github/workflows/version_check.yml`).
