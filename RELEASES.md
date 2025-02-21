# Grimoire CSS Releases

## Overview

This document combines all release notes in chronological order, providing a comprehensive view of Grimoire CSS's evolution.

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

---

# v1.0.0: Initial Release

The debut release of Grimoire CSS, introducing a powerful CSS system engine designed for flexibility and performance.
