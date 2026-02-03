# Grimoire CSS Releases

## Overview

This document combines all release notes in chronological order, providing a comprehensive view of Grimoire CSS's evolution.

---

# v1.8.0 Runebridge

Grimoire CSS expands both its **core composition model** and its **tooling surface** with **Runebridge**. This release improves scroll composition by adding argument-driven spell selection (`spellByArgs`) and enabling parameterized scroll invocations inside `scrolls[].spells` (e.g. `box=12px`). On top of that, it introduces a Rust **Analyzer**, a feature-gated **LSP server** (stdio), and a first-party **VS Code extension**.

## Key Highlights

- **Scroll composition upgrades**: `spellByArgs` for argument-driven spell selection and parameterized scroll usage inside `scrolls[].spells`.
- **Analyzer (Rust)**: Indexes projects, resolves references, computes stats, runs lints, and explains tokens in a structured, IDE-friendly format.
- **LSP Server (stdio)**: Feature-gated `grimoire_css_lsp` exposes analyzer capabilities via `workspace/executeCommand` for editor integration.
- **VS Code Extension**: A full UI layer (views + actions) with hover insights, highlights, and navigation for spells/scrolls/variables.
- **Scriptable CLI (`fi`)**: Machine-friendly JSON outputs for automation and for sharing the same "commands" between CLI and LSP.
- **Core improvements**: Parser/token extraction hardening, stricter spell validation, config/schema alignment, and optimizer controls.
- **Dev Experience**: Extension CI workflows and repo VS Code tasks/launch configs to make local development repeatable.
- **Additional Binary Name**: The main CLI binary is now also available as `grim` (alongside `grimoire_css`).
- **Updated MSRV**: Minimum supported Rust version is now 1.93.0.

## Full Details

### Scroll Composition Upgrades (Core)

Runebridge improves scroll authoring and composition in core configuration:

- **`spellByArgs`**: Choose spell sets based on invocation arguments for more expressive, reusable scrolls.
- **Parameterized scroll invocations inside `scrolls[].spells`**: You can now put invocations like `box=12px` directly into `scrolls[].spells`.

  This matters because `extend` only supports inheriting by **scroll name**, while many real-world scrolls are _parameterized_. Example: you may have a scroll `box` that accepts args (`box=12px`). Previously you could not create a new scroll whose `spells` include `box=12px`; now you can.

These changes make large style systems easier to model and enable richer analysis/refactoring in IDE tooling.

### Analyzer: Structured CSS Spell Intelligence

The new analyzer provides a reusable set of operations designed to be consumed by both the CLI and IDE clients:

- **Index / Explorer data**: A deterministic snapshot of key entities and their relationships.
- **References**: Find usage of spells, scrolls, and Grimoire variables.
- **Stats**: Usage counts and "what is this token" hints.
- **Explain**: Parse a token into meaningful parts and return structured diagnostics on invalid tokens.
- **Lint**: Detect suspicious or unused configuration-driven entities (policy-based).
- **Dry candidates**: Identify refactoring candidates and optionally generate scroll suggestions.

### LSP Server: IDE Commands over stdio

Runebridge introduces `grimoire_css_lsp` (enabled with `--features lsp`), designed to be embedded by editor extensions.

It focuses on `workspace/executeCommand` to expose analyzer functionality in a stable, versioned command surface.

Canonical command set includes (may evolve):

- `grimoirecss.explorerIndex`, `grimoirecss.entityDetails`
- `grimoirecss.index`, `grimoirecss.lint`, `grimoirecss.refs`, `grimoirecss.explain`, `grimoirecss.stats`
- `grimoirecss.configSummary`, `grimoirecss.documentSpells`
- `grimoirecss.dryCandidates`, `grimoirecss.dryCreateScroll`
- `grimoirecss.listScrolls`, `grimoirecss.listVars`

### VS Code Extension: First-party Editor Integration

The VS Code extension provides a UI and workflow layer on top of the LSP server:

- **Views**: Explorer, Details, Config, Actions, Refs, Problems/Diagnostics, and DRY helpers.
- **Editor UX**: Hover explain, highlights, and code actions that connect tokens to details and refs.
- **Workspace-aware**: Supports selecting the correct root in multi-folder setups.

### `fi`: Scriptable CLI for Automation

The `fi` command provides JSON output suitable for scripts, CI, and editor tooling. It intentionally mirrors LSP capabilities so that CLI and IDE can share the same mental model and (where appropriate) the same command semantics.

### Core Improvements

This release includes meaningful core work that improves correctness and clarity, and also benefits analysis tooling:

- **Parser improvements for token extraction**: better handling of templated tokens, spans/offsets, and edge cases needed for precise refs and hover.
- **Spell parsing/validation tightening**: malformed function-like targets fail earlier with clearer errors.
- **Config + schema alignment**: config FS loading and JSON schema updates to keep tooling outputs consistent.
- **Optimizer controls**: added a path for readable (non-minified) CSS output when tooling needs "pretty" results.

## Migration Notes

### For Users

- **No required workflow changes**: core CSS generation remains available via the standard CLI and crate APIs.
- **Optional IDE stack**: LSP is behind a feature flag; you can avoid its dependencies by not enabling `lsp`.

### For Contributors

- **MSRV**: Development targets Rust 1.93.0+.
- **Extension tooling**: VS Code extension workflows run on Node 20.

---

# v1.7.1 Scryforge Hotfix

Grimoire CSS v1.7.1 is a small **stability patch** on top of **Scryforge**. It fixes a parser edge case where templated spell detection could accidentally match `g!…;` across token boundaries in certain code patterns (notably Rust macros like `debug!`).

## Key Highlights

- **Safer `g!…;` detection**: templated spells are only collected at token boundaries to avoid false positives.
- **Better mixed-code parsing**: reduces accidental spell collection when scanning Rust/JS/TS sources that contain macro-like identifiers.
- **No migration required**: no config changes, same output for valid inputs.

## Full Details

### Templated Spell Boundary Fix

The templated-spell matcher now requires a non-identifier boundary before `g!…;` (or start-of-content). This prevents cases where a preceding character sequence could cause `g!` detection to fire inside unrelated tokens.

A regression test was added to ensure Grimoire CSS does not collect templated spells from Rust macro invocations ending with `g` (e.g. `log::debug!(...)`).

## Migration Notes

- No action needed. If you previously hit unexpected spell extraction in mixed Rust code, upgrading to v1.7.1 should fix it.

---

# v1.7.0 Scryforge: Deterministic Scrollcraft

Grimoire CSS sharpens both **power** and **clarity** with **Scryforge** — a release focused on templated scroll composition, rustc-like diagnostics, and measurable performance wins. Spells now expand more deterministically under templated selectors, errors read like a compiler, and large projects build faster with an opt-in multi-core path.

## Key Highlights

- **Scroll Templates Inside `g!…;`**: Use config-defined `scrolls` directly in `g!…;` with variable arguments (e.g. `g!complex-card=120px_red_100px;`) while keeping output deterministic.
- **Prefix-Safe Expansion**: Prefix modifiers like `md__`, focus blocks `{...}`, and `hover:` are preserved by applying them to each expanded spell.
- **Rustc-like Diagnostics**: Errors render with file context + labeled span + optional help text, powered by a structured error model and `miette` diagnostics.
- **Faster Builds (Same Output)**: Reduced redundant work and clone/allocation pressure; optional parallel project builds via `GRIMOIRE_CSS_JOBS`.
- **Better Repro & Contributor UX**: Added a `repro/` sandbox for feature/error scenarios and added `.github/copilot-instructions.md` as an architecture guide.

## Full Details

### Scroll Templates Inside `g!…;`

You can now reference scrolls in templated `g!…;` syntax, including argument passing:

- Example: `g!complex-card=120px_red_100px;`
- Supports variable-like arguments and prefix modifiers.
- Scroll expansion is flattened into real property spells so generated CSS is emitted under the *outer templated selector*.
- Output remains deterministic and the template-flattening path avoids unnecessary cloning.

### Prefix Semantics Preserved

Scrolls expanded under templates keep the semantics that made Grimoire CSS predictable in complex UIs:

- Responsive prefixes like `md__`.
- Focus blocks using `{...}`.
- Effects like `hover:`.

These prefixes apply to each expanded spell during flattening so behavior matches user intent.

### Rustc-like Diagnostics (File + Span)

This release substantially upgrades the error/reporting system:

- Introduced `SourceFile` to carry file identity + full content, enabling readable snippets for every error.
- Parsing now tracks spans (`start`, `len`) for each extracted class/spell token and propagates them through spell generation.
- Error model upgraded from plain strings to structured compile errors (message / label / help / span / source).
- Added a diagnostics adapter mapping `GrimoireCssError` to `miette::Diagnostic` for polished CLI output.

User-facing validation got stricter and clearer:

- Better errors for malformed function-like values / parentheses (`spell_value_validator`).
- Color function argument validation now returns a proper error instead of being silently ignored.

### Performance Improvements + Opt-in Parallelism

Grimoire CSS stays output-stable while getting faster and leaner:

- Reduced redundant passes and duplicated work.
- Lowered allocation and clone pressure in hot paths.
- Added opt-in parallelism for filesystem builds via `GRIMOIRE_CSS_JOBS`.

Safe default remains single-threaded; scaling is opt-in based on machine and project size.

### Repro Sandbox + Architecture Guide

To improve maintenance and debugging velocity:

- Added `repro/` containing minimal scenarios for reproducing features and diagnostics.
- Added `.github/copilot-instructions.md` documenting the project’s architecture conventions.

## Migration Notes

### For Users

- **Optional parallel builds**: Set `GRIMOIRE_CSS_JOBS` to enable multi-core builds in filesystem mode. Without it, behavior remains unchanged.
- **Stricter validation**: Invalid color function arguments that were previously ignored now raise proper errors (with spans and help text).

### For Contributors

- Prefer adding/using minimal scenarios under `repro/` when improving parser/diagnostics behavior.
- Follow the architecture guide in `.github/copilot-instructions.md` when introducing new commands, core pipeline changes, or infrastructure glue.

---

# v1.6.0 Chromaspire: The Color Convergence

Grimoire CSS refines its arcane precision with **Chromaspire**, a release dedicated to mastery over color and stability. With a fully decoupled color toolkit, improved resilience, and groundwork for precise float-based styling, spellcasters now wield both grace and robustness.

## Key Highlights

- **Decoupled Color System**: Color module extracted to `grimoire_css_color_toolkit` crate for independent usage and improved maintainability.
- **Enhanced Parser Support**: Comprehensive support for curly bracket class syntax (`class={}`, `className={}`) with robust nested bracket handling.
- **Float-Based Precision**: Migration from `u32` to `f64` for unit handling, enabling precise floating-point calculations for responsive units.
- **Modern Rust Standards**: Upgraded to Rust Edition 2024 with minimum version 1.88 for enhanced language features and performance.
- **Improved String Formatting**: Modernized string interpolation using the latest Rust formatting conventions.
- **Enhanced Documentation**: Refined project description and branding consistency across the ecosystem.
- **Robust Refactoring Support**: Added comprehensive support for refactor branches in contribution workflow.

## Full Details

### Color System Decoupling

- **Independent Color Toolkit**: Extracted the complete color module to `grimoire_css_color_toolkit` v1.0.0 as a standalone crate.
- **CSS Color Module Level 4 Compliance**: Maintained full compliance with CSS Color specifications for `rgb()`, `hsl()`, `hwb()`, hex codes, and named colors.
- **External Availability**: Color toolkit now available for independent use in other projects requiring CSS-compliant color parsing and manipulation.
- **Seamless Integration**: Existing color functionality remains fully accessible through the main Grimoire CSS module.

### Parser Enhancements

- **Curly Bracket Class Support**: Added comprehensive parsing for `class={}` and `className={}` syntax patterns.
- **Nested Bracket Handling**: Robust regex patterns that correctly handle nested curly brackets within class declarations.
- **Framework Compatibility**: Enhanced support for modern JavaScript frameworks and CSS-in-JS solutions.
- **Collection Type Management**: Implemented `CollectionType` enum for precise handling of different class collection scenarios.

### Precision and Performance

- **Float-Based Units**: Migrated unit handling from `u32` to `f64` for precise floating-point calculations.
- **Responsive Design Support**: Enhanced accuracy for `mfs`/`mrs` (minimum/maximum font-size) and other fluid sizing calculations.
- **Mathematical Precision**: Improved handling of complex responsive calculations with decimal precision.
- **Memory Efficiency**: Optimized data structures while maintaining calculation accuracy.

### Language and Tooling Modernization

- **Rust Edition 2024**: Upgraded to the latest Rust edition for access to newest language features and optimizations.
- **Minimum Rust Version**: Set minimum supported Rust version to 1.88 for stability and security.
- **Modern String Formatting**: Migrated to contemporary Rust string interpolation patterns for improved readability and performance.
- **Dependency Updates**: Updated Clap to v4.5.41 with enhanced CLI argument parsing capabilities.

### Documentation

- **Consistent Terminology**: Standardized project description as "A magical CSS engine" across all documentation.
- **Enhanced Contributing Guidelines**: Added comprehensive support for refactor branches in the development workflow.

## Migration Notes

### For Library Users

- **Color Module**: If using color functions directly, update imports to reference the new `grimoire_css_color_toolkit` crate or continue using through the main module.
- **Unit Calculations**: Float-based calculations may produce slightly different results due to improved precision.
- **No Breaking Changes**: All existing APIs remain compatible with previous versions.

### For Contributors

- **Refactor Branches**: New `refactor/{description}` branch naming convention available for code improvement contributions.
- **Modern Rust**: Development now requires Rust 1.88+ for optimal experience.
- **Testing**: Enhanced test coverage for curly bracket parsing and float-based calculations.

## Technical Improvements

### Core Architecture

- **Modular Design**: Color system decoupling improves overall architecture and reduces main crate complexity.
- **Type Safety**: Enhanced type safety with `CollectionType` enum for parser state management.
- **Error Handling**: Improved error handling for complex parsing scenarios.
- **Code Organization**: Better separation of concerns between core functionality and specialized modules.

### Parser Robustness

- **Regex Optimization**: Efficient regex patterns for curly bracket class detection with proper nesting support.
- **Quote Handling**: Enhanced quote detection and matching for complex class declarations.
- **State Management**: Improved parser state tracking for reliable multi-pattern matching.
- **Performance**: Optimized parsing pipeline for faster processing of complex markup.

---

# v1.5.0 Arcane Nexus: Unified Spellcraft

Grimoire CSS continues its magical ascendance with the **v1.5.0 Arcane Nexus** release, forging powerful new commands, extensible configurations, advanced template syntax, and a unified ecosystem that binds the circle of spellcasters together. This update focuses on seamless migration, modular scroll and variable support, a public color toolkit, and a next-generation Transmutator CLI & web UI - all while refining performance, parsing reliability, and community engagement.

## Key Highlights

- **`shorten` Command**: Compress and simplify all spells in filesystem projects with a single CLI invocation and detailed summary output.
- **External Scrolls & Variables**: Load and merge `grimoire.*.scrolls.json` and `grimoire.*.variables.json` for modular, shareable style collections.
- **Template Spell Grouping**: Define multi-spell templates using the `&` delimiter (e.g., `g!c=purple&disp=flex;`) to streamline CSS-in-JS workflows.
- **Color Toolkit**: Public Grimoire CSS Color module now available for parsing and manipulating colors per CSS Color Module Level 4.
- **Media Query Cascade**: Media queries are now emitted at the end of generated CSS to follow the natural cascade and improve override control.
- **Refined Short Components**: Updated shorthand properties with consistent naming, logical conventions, and improved syntax clarity.
- **Parser Reliability**: Quote-aware parsing fixes ensure correct handling of single vs. double quotes within spells and string values.
- **Performance & Optimizations**: Continued algorithmic enhancements across core processing, further reducing memory usage and build times.
- **Mascot Introduction**: Meet **Grimm**, the official wizard mascot of Grimoire CSS.
- **Ecosystem & Transmutator**: The `gcsst` CLI has evolved into the **Transmutator**, offering 2× performance gains, file and in-memory processing, native external config output, and a brand-new web UI.
- **Community Hub & Platform**: Launched **grimoirecss.com**, the live Playground, the Transmutator web interface, and the **Circle** for sharing configs, scrolls, variables, and UI kits.
- **Tailwind & Benchmarking**: Full external scrolls for Tailwind CSS (static utilities) plus comprehensive benchmark charts comparing Grimoire CSS v1.5.0 vs Tailwind v4.x.

## Full Details

### CLI Enhancements

- Introduced the `shorten` command (FS mode only) to batch-compress spells and display a concise summary of before/after changes.
- Improved overall argument parsing and execution flow for consistent library and CLI experiences.

### External Scrolls & Variables

- Support for loading external JSON files matching `grimoire.*.scrolls.json` and `grimoire.*.variables.json`.
- Enables modular style libraries, third-party plugin scrolls, and shareable variable sets without code changes.

### Template Spell Grouping

- Extended template syntax to allow multiple spells in a single template using the ampersand (`&`) delimiter.
- Simplifies embedding complex style groups in CSS-in-JS scenarios with minimal boilerplate.

### Color Module Public Release

- Opened the standalone Grimoire CSS Color core for external use as a color parsing & manipulation toolkit.
- Full compliance with CSS Color Module Level 4: `rgb()`, `hsl()`, hex, named colors, and more.

### Parsing & Short Component Refinements

- Media queries are now generated at the end of output CSS to maintain a predictable cascade.
- Overhauled short component definitions with improved naming conventions and consistent property mappings.
- Enhanced templated spell parsing for greater flexibility within template blocks.
- Fixed parser logic to detect initial quote type and correctly identify matching closing quotes.

### Mascot & Branding

- Introduced **Grimm**, the wizard mascot of Grimoire CSS, to embody the project’s spirit and guide new users through the Arcane Circle.

### Ecosystem & Transmutator CLI

- `gcsst` utility rebranded as **Transmutator**, with 2× performance improvements, full CSS support, and both filesystem & in-memory modes.
- Native JSON config output aligns with external scrolls convention by default.
- Replaced `gcsst-ui` with a modern web interface featuring enhanced visualization and workflow controls.

### Community Platform & Resources

- Launched the official website **grimoirecss.com** with documentation, guides, and download links.
- Live in-browser **Playground** for interactive Grimoire CSS experimentation.
- **Circle** community portal for sharing configs, scrolls, variables, components, and UI kits.

### Tailwind CSS Integration & Benchmarks

- Added full external scrolls support for Tailwind CSS static utilities under `grimoire.tailwindcss.scrolls.json` for zero-markup migration.
- Benchmarking suite updated with detailed charts comparing build time, memory usage, CPU performance, and output size against Tailwind CSS v4.x.

---

# v1.4.0 Aetheric Flow: Refined Spellcasting

Grimoire CSS enhances its magical arsenal with the **v1.4.0 Aetheric Flow** release, bringing refined argument handling, enhanced visual feedback through new spinners, and improved CLI flow. This update focuses on the quality-of-life improvements that make spell-casting (development) more intuitive and visually engaging.

## Key Highlights

- **Enhanced Argument Handling**: Improved flexibility with `Vec<String>` for better NodeJS integration
- **Expanded Visual Feedback**: New spinners for better progress visualization
- **Refined CLI Experience**: Streamlined start_as_cli flow for more intuitive operation
- **Unified Documentation**: New combined RELEASES.md for complete release history

## Full Details

### Enhancements

#### Argument Handling Improvements
- Replaced `&[String]` with `Vec<String>` for more flexible argument processing
- Enhanced compatibility with NodeJS wrapper implementation
- Improved argument collection and processing through `env::args()`

#### Visual Feedback Enhancement
- Added new spinner variations for different operation states
- Enhanced progress visualization during lengthy operations
- Improved user experience with more engaging loading indicators

#### CLI Flow Optimization
- Streamlined `start_as_cli` workflow for better usability
- Enhanced command processing and execution flow
- Improved overall CLI interaction experience

### Internal Changes

- **API Refinements**
  - Updated argument type signatures for better integration
  - Optimized argument handling in core functions

### Documentation

- **Release History Enhancement**
  - Introduced RELEASES.md to provide a comprehensive view of all releases
  - Combined all release notes in chronological order for better project history tracking
  - Enhanced accessibility of historical changes and updates

---

# v1.3.0 Liminal: Beyond the Threshold

Grimoire CSS breaks free from filesystem constraints with the **v1.3.0 Liminal release**, introducing in-memory processing capabilities and significant performance enhancements. This transformative update enables seamless integration with serverless environments and web frameworks while delivering an improved developer experience.

## Key Highlights

- **In-Memory Processing**: Process CSS entirely in memory for runtime and serverless environments.
- **Enhanced Configuration**: Flexible configuration management with improved browserslist support.
- **Performance Boost**: Significant optimizations across CSS generation and file operations.
- **Refined CLI Experience**: More intuitive and informative command-line interface.

## Full Details

### Enhancements

- **In-Memory Mode**

  - Introduced `ConfigInMemory` for configuring in-memory operations.
  - Implemented `CssBuilderInMemory` for memory-based CSS processing.
  - Added `build_in_memory` function for flexible I/O operations.
  - Enhanced support for alternative storage solutions.

- **Configuration Improvements**

  - Restructured configuration with distinct `ConfigFs` and `ConfigInMemory` structures.
  - Enhanced browserslist handling with `.browserslistrc` support.
  - Improved configuration file organization and clarity.

- **Performance Optimizations**

  - Replaced global static `MESSAGE_BUFFER` with thread-local `RefCell`.
  - Improved CSS generation efficiency.
  - Optimized file system operations for better performance.

- **CLI Experience**

  - Enhanced build progress UI with clearer feedback.
  - Improved error reporting and debugging experience.
  - Added comprehensive logging for better visibility.

### Documentation

- Updated `README` with in-memory processing examples
- Enhanced configuration documentation
- Added serverless deployment guidelines

### Architecture

- Introduced `CssBuilderBase` for shared functionality
- Modernized codebase structure
- Improved code maintainability

---

# v1.2.0 Emberveil: Transformative Styling

Grimoire CSS introduces a transformative set of features with the v1.2.0 Emberveil release, enhancing adaptive styling, dynamic color manipulation, and build efficiency. This update solidifies Grimoire's position as the CSS engine for developers who seek both power and flexibility.

## Key Highlights

- **Fluid Sizing with mfs**: Seamless, adaptive styles without media queries.
- **Built-in Color Functions**: Dynamic color manipulation adhering to CSS standards.
- **Project Locking:** Streamlined builds with automatic cleanup of outdated files.
- **Extended CLI Support**: Enhanced metrics and logging in library and CLI modes.

## Full Details

### Enhancements

- **Fluid Size Functionality ([#14](https://github.com/persevie/grimoire-css/issues/14))**

  - Introduced the `mfs` (Make Fluid Size) function for adaptive styling, eliminating reliance on media queries for fluid designs.
  - Improved `handle_grimoire_functions` to support future scalable enhancements.

- **Built-in Color Functions ([#15](https://github.com/persevie/grimoire-css/issues/15))**

  - Added dynamic color transformations such as lighten, darken, mix, and more.
  - Developed a standalone color core module compliant with CSS Color Module Level 4.
  - Enabled parsing and manipulation of CSS-standard color notations (e.g., rgb, hsl, hex).

- **Project Locking for Build Optimization ([#16](https://github.com/persevie/grimoire-css/issues/16))**

  - Introduced an optional `grimoire.lock.json` file for tracking and cleaning up obsolete builds.
  - Enabled via `lock: true` in `grimoire.config.json`, defaulting to false for backward compatibility.

- **Improved Logging and Metrics ([#43](https://github.com/persevie/grimoire-css/issues/43))**

  - Added `start_as_cli` function to provide library-level access to CLI features like logging and execution timing.
  - Ensured consistency between CLI and library outputs for seamless integration.

### Documentation

- Updated README with new features

### Chore

- New LICENSE

---

# v1.1.0 Arcana: Unleashing Core Magic

Grimoire CSS takes a major leap forward with the **v1.1.0 Arcana** release, bringing **double the performance**, hundreds of new animations, and significant ecosystem expansions. This update introduces **Grimoire CSS JS (gcssjs)**, a dedicated JavaScript wrapper for seamless integration with the JavaScript ecosystem, alongside plugins for popular bundlers like Vite, Webpack, and Rollup. All related work for **gcssjs** and its plugins is now managed in a separate repository with its own **GitHub Projects board** for better organization and visibility.

## Key Highlights

- **2x Performance Boost** thanks to optimized processing.
- **632 New Animations** added to the core collection.
- **Introduction of Grimoire CSS JS** and plugins for modern JavaScript build tools.
- Comprehensive updates to documentation for easier onboarding.

## Full Details

### Enhancements

- **Enhanced Components Initialization ((#8)[https://github.com/persevie/grimoire-css/issues/8])**

  - Optimized initialization to improve performance by ensuring components initialize only once.
  - Restructured component architecture for greater flexibility.
  - Introduced a two-way dictionary to support both full and shorthand syntax options.

- **Improved Error Messaging and User Feedback ((#10)[https://github.com/persevie/grimoire-css/issues/10])**

  - Enhanced error messages and feedback mechanisms for a better user experience.

- **Parallel Execution Improvements ((#11)[https://github.com/persevie/grimoire-css/issues/11])**
  - Refactored CSSBuilder and Parser to process files sequentially, eliminating unnecessary parallelism (removed rayon dependency).
  - Performance has **doubled** in scenarios involving large-scale projects, thanks to recent optimizations.

### New Ecosystem: Grimoire CSS JS

- **Grimoire CSS JS (gcssjs) ([#4](https://github.com/persevie/grimoire-css/issues/4))**:
  - Introduced a JavaScript wrapper for Grimoire CSS, enabling seamless integration into the JavaScript ecosystem.
- **Plugins for Bundlers**:
  - Released plugins for Webpack ([#5](https://github.com/persevie/grimoire-css/issues/5)), Vite ([#6](https://github.com/persevie/grimoire-css/issues/6)), and Rollup ([#7](https://github.com/persevie/grimoire-css/issues/7)).
- **Separate Repository**:
  - All work related to Grimoire CSS JS and its plugins is now maintained in a dedicated repository with its own **GitHub Projects board**.

### Bug Fixes

- **Parser Regex Fix ([#28](https://github.com/persevie/grimoire-css/issues/28))**
  - Improved regex patterns to support all types of quotes.

### Documentation

- Updated README with:
  - Recent features.
  - Installation instructions.
  - Usage examples.
  - Basic configuration guidelines.
  - Improved logo.
