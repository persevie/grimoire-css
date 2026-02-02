# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This file is auto-generated from per-version release notes in `releases/`.
Do not edit it manually — edit the corresponding file in `releases/` and re-run the generator.

## [Unreleased]

(no unreleased changes recorded)

## [v1.8.0] - Unreleased

> Full release notes: [releases/v1.8.0.md](./releases/v1.8.0.md)

### Added

- Scroll composition upgrades: argument-driven spell selection via `spellByArgs` and parameterized scroll invocations inside `scrolls[].spells` (e.g. `box=12px`).
- Rust Analyzer powering structured operations like index/explorer data, refs, stats, explain, lint, and DRY candidates.
- Feature-gated stdio LSP server (`grimoire_css_lsp`) exposing analyzer capabilities via `workspace/executeCommand`.
- First-party VS Code extension with views, navigation, highlights, and tooling actions.
- Scriptable CLI surface (`fi`) with machine-friendly JSON output (aligned with LSP commands).
- Additional CLI binary name: `grim` (alongside `grimoire_css`).

### Changed

- Hardened parsing/token extraction and tightened spell validation for more consistent diagnostics.
- Improved config/schema alignment for tooling and IDE workflows.
- Added optimizer controls to support readable (non-minified) CSS output for tooling.
- Updated MSRV to Rust 1.93.0.

## [v1.7.1] - 2026-01-19

> Full release notes: [releases/v1.7.1.md](./releases/v1.7.1.md)

### Fixed

- Templated spell detection: only match `g!…;` at token boundaries (prevents false positives in Rust macros like `debug!`).

## [v1.7.0] - 2025-12-27

> Full release notes: [releases/v1.7.0.md](./releases/v1.7.0.md)

### Added

- **Scroll templates in `g!…;`**: Use config-defined `scrolls` inside templated syntax with variable arguments.
- **Rustc-like diagnostics**: File/snippet output with labeled spans and optional help text.
- **Opt-in parallel builds**: Enable multi-core filesystem builds via `GRIMOIRE_CSS_JOBS`.
- **Repro sandbox**: Added `repro/` scenarios for quickly validating features and diagnostics.
- **Contributor instructions**: Added `.github/copilot-instructions.md` describing the repo’s architecture conventions.

### Changed

- Deterministic scroll expansion under templated selectors, including correct propagation of prefixes (`md__`, `{...}`, `hover:`).
- Reduced redundant work and lowered clone/allocation pressure in hot paths (output unchanged).

### Fixed

- Malformed function-like spell values now produce clearer, earlier errors.
- Color function argument validation now returns a proper error instead of being silently ignored.

## [v1.6.0] - 2025-07-21

> Full release notes: [releases/v1.6.0.md](./releases/v1.6.0.md)

### Added

- Extracted the color module into `grimoire_css_color_toolkit` for independent usage.
- Comprehensive support for curly-bracket class syntax (`class={}`, `className={}`) with nested bracket handling.

### Changed

- Migrated unit handling from `u32` to `f64` for better precision in responsive calculations.
- Upgraded to Rust Edition 2024 and set MSRV to Rust 1.88.

## [v1.5.0] - 2025-05-20

> Full release notes: [releases/v1.5.0.md](./releases/v1.5.0.md)

### Added

- **shorten Command**: Compress and simplify all spells in filesystem projects with a single CLI invocation and detailed summary output.
- **External Scrolls & Variables**: Load and merge `grimoire.*.scrolls.json` and `grimoire.*.variables.json` for modular, shareable style collections.
- **Template Spell Grouping**: Define multi-spell templates using the `&` delimiter for streamlined CSS-in-JS workflows.
- **Color Toolkit**: Public Grimoire CSS Color module for parsing and manipulating colors per CSS Color Module Level 4.
- **Mascot Introduction**: Meet Grimm, the official wizard mascot of Grimoire CSS.
- **Transmutator CLI & Web UI**: `gcsst` utility rebranded as Transmutator, with 2× performance improvements, new web UI, and native config output.
- **Community Platform**: Launched grimoirecss.com, Playground, and Circle community portal.
- **Tailwind CSS Integration**: Full external scrolls for Tailwind CSS static utilities and comprehensive benchmarks.

### Changed

- Media queries are now emitted at the end of generated CSS for better cascade control.
- Refined short component definitions with improved naming and property mappings.
- Enhanced parser reliability, especially for quote handling.
- Continued performance and memory optimizations.

### Fixed

- Quote-aware parsing fixes for correct handling of single vs. double quotes.
- Parser logic improvements for template and spell parsing.

## [v1.4.0] - 2025-02-21

> Full release notes: [releases/v1.4.0.md](./releases/v1.4.0.md)

### Added

- Enhanced argument handling with `Vec<String>` for better NodeJS integration.
- New spinner variations for improved progress visualization.
- Unified documentation: new combined RELEASES.md for complete release history.

### Changed

- Streamlined CLI flow with improved `start_as_cli` workflow.
- Enhanced command processing and execution flow.
- Improved overall CLI interaction experience.
- Updated argument type signatures for better integration.
- Optimized argument handling in core functions.

## [v1.3.0] - 2025-02-21

> Full release notes: [releases/v1.3.0.md](./releases/v1.3.0.md)

### Added

- **In-Memory Processing**: CSS processing without filesystem dependencies
- **ConfigInMemory**: New configuration system for in-memory operations
- **Enhanced Logging**: More comprehensive build progress feedback

### Changed

- Performance optimizations with thread-local message handling
- Configuration management with better browserslist support
- CLI user experience with clearer feedback
- Code architecture with `CssBuilderBase` introduction

## [v1.2.0] - 2024-12-23

> Full release notes: [releases/v1.2.0.md](./releases/v1.2.0.md)

### Added

- **`mfs` Function**: Introduced adaptive fluid sizing ([#14](https://github.com/persevie/grimoire-css/issues/14)).
- **Built-in Color Functions**: Enabled dynamic color manipulation and CSS-compliant parsing ([#15](https://github.com/persevie/grimoire-css/issues/15)).
- **Project Locking**: Automatic cleanup of old builds with `grimoire.lock.json` ([#16](https://github.com/persevie/grimoire-css/issues/16)).
- **Extended Logging and Metrics**: Unified CLI and library logging features ([#43](https://github.com/persevie/grimoire-css/issues/43)).

### Changed

- Modularized internal handling of `mfs` for scalability.
- Streamlined build process for project-specific configurations.

## [v1.1.0] - 2024-11-23

> Full release notes: [releases/v1.1.0.md](./releases/v1.1.0.md)

### Added

- Grimoire CSS JS: JavaScript wrapper for seamless integration.
- Plugins for Vite, Webpack, Rollup.
- 632 new animations.

### Changed

- Component initialization performance (2x boost).
- Enhanced error messaging.

### Fixed

- Improved regex patterns to support all types of quotes.

## [v1.0.0] - 2024-10-21

> Full release notes: [releases/v1.0.0.md](./releases/v1.0.0.md)

**Initial Release**

Grimoire CSS debuts as a powerful CSS engine designed for flexibility and performance.
