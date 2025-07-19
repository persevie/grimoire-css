# Changelog

## [v1.5.0] - 2025-05-19

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

### Improved

- Media queries are now emitted at the end of generated CSS for better cascade control.
- Refined short component definitions with improved naming and property mappings.
- Enhanced parser reliability, especially for quote handling.
- Continued performance and memory optimizations.

### Fixed

- Quote-aware parsing fixes for correct handling of single vs. double quotes.
- Parser logic improvements for template and spell parsing.

---

## [v1.4.0] - 2025-04-10

> Full release notes: [releases/v1.4.0.md](./releases/v1.4.0.md)

### Added

- Enhanced argument handling with `Vec<String>` for better NodeJS integration.
- New spinner variations for improved progress visualization.
- Unified documentation: new combined RELEASES.md for complete release history.

### Improved

- Streamlined CLI flow with improved `start_as_cli` workflow.
- Enhanced command processing and execution flow.
- Improved overall CLI interaction experience.
- Updated argument type signatures for better integration.
- Optimized argument handling in core functions.

---

## [v1.3.0] - 2025-02-21

> Full release notes: [releases/v1.3.0.md](./releases/v1.3.0.md)

### Added

- **In-Memory Processing**: CSS processing without filesystem dependencies
- **ConfigInMemory**: New configuration system for in-memory operations
- **Enhanced Logging**: More comprehensive build progress feedback

### Improved

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

### Improved

- Modularized internal handling of `mfs` for scalability.
- Streamlined build process for project-specific configurations.

---

## [v1.1.0] - 2024-11-23

> Full release notes: [releases/v1.1.0.md](./releases/v1.1.0.md)

### Added

- Grimoire CSS JS: JavaScript wrapper for seamless integration.
- Plugins for Vite, Webpack, Rollup.
- 632 new animations.

### Improved

- Component initialization performance (2x boost).
- Enhanced error messaging.

### Fixed

- Improved regex patterns to support all types of quotes.

---

## [v1.0.0] - 2024-10-21

**Initial Release**

Grimoire CSS debuts as a powerful CSS engine designed for flexibility and performance.
