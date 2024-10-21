<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**

- [Contributing to Grimoire CSS](#contributing-to-grimoire-css)
  - [Development Workflow](#development-workflow)
    - [Main Branches](#main-branches)
    - [Roadmap and Contribution Opportunities](#roadmap-and-contribution-opportunities)
    - [Feature Development](#feature-development)
    - [Release Process](#release-process)
  - [Setting Up the Project Locally](#setting-up-the-project-locally)
  - [Code Quality Standards](#code-quality-standards)
  - [Code Style Guidelines](#code-style-guidelines)
  - [Commit Message Guidelines](#commit-message-guidelines)
    - [Format:](#format)
    - [Example Commit Messages:](#example-commit-messages)
  - [Pull Request Guidelines](#pull-request-guidelines)
  - [Contribution Checklist](#contribution-checklist)
  - [Issue Reporting and Discussion](#issue-reporting-and-discussion)
  - [License](#license)
  - [Contributor Expectations](#contributor-expectations)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# Contributing to Grimoire CSS

Thank you for considering contributing to Grimoire CSS! We welcome all forms of contribution: code, documentation, examples, and more. This guide outlines the process and standards for contributing to the project.

## Development Workflow

We follow a custom Git Flow approach to manage the lifecycle of code and contributions. Below is an overview of the main branches and how to contribute within this flow.

### Main Branches

- **`main`**: The stable branch where production-ready code resides. Only thoroughly tested and approved code is merged here.
- **`develop`**: The active development branch. All new features and changes are merged into `develop` via pull requests. Testing is required before merging into `main`.

### Roadmap and Contribution Opportunities

We maintain a [Roadmap](./ROADMAP.md) to help contributors understand the project's goals and priorities. Before starting to work on a new feature, please check the roadmap for guidance on high-priority items.
For task assignment and bug tracking, refer to our [Issues tab](https://github.com/persevie/grimoire-css/issues). Here, you can find bugs, feature requests, and other tasks open for contribution. If you find an issue you would like to work on, please comment on it or create a new issue if it doesn’t already exist.

> We are considering using GitHub Projects to track progress, milestones, manage work, and maintain the roadmap.

### Feature Development

1. **Create a new branch**:
   - For new features or bug fixes, create a branch from `develop` using the following naming conventions:
     - For features: `feature/your-feature-name`
     - For bug fixes: `fix/your-bugfix-description`
2. **Write clean, modular code**: Follow the code quality standards mentioned below.
3. **Commit your changes**: Use [conventional commits](https://www.conventionalcommits.org/) to write meaningful commit messages.
4. **Pull Request to `develop`**: Once your changes are ready, open a pull request to the `develop` branch. Ensure your code passes all checks before requesting a review.

### Release Process

- Once the `develop` branch contains all necessary features and bug fixes for a release, a **release branch** is created (e.g., `release/v1.0.0`).
- Final testing and minor fixes are done in this branch before merging into `main` and tagging the release.
- If critical issues are found in production, a **hotfix branch** (e.g., `hotfix/v1.0.1`) is created from `main` and later merged into both `main` and `develop`.

## Setting Up the Project Locally

To contribute to Grimoire CSS, you'll need to set up the project on your local machine. Follow these steps:

1. **Clone the repository**:
   ```bash
   git clone git@github.com:persevie/grimoire-css.git
   cd grimoire-css
   ```

2. **Install Rust** (if you don't have it installed already):
   Follow the official [Rust installation guide](https://www.rust-lang.org/tools/install).

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run the tests** to make sure everything is set up correctly:
   ```bash
   cargo test
   ```

After this setup, you're ready to start contributing to the project.

## Code Quality Standards

Maintaining high-quality code is crucial for the stability and maintainability of the project. Please follow these guidelines:

1. **Code Formatting**: All code should be formatted using `cargo fmt`. Our CI pipeline ensures that improperly formatted code will fail the build.
2. **Linting**: Use `cargo clippy` to ensure your code is free of warnings. We treat warnings as errors (`cargo clippy -- -D warnings`).
3. **Tests**: All new features or fixes should include appropriate unit and/or integration tests. Ensure tests pass before submitting a pull request (`cargo test`).

## Code Style Guidelines

To maintain consistency across the project, please adhere to the following code style guidelines:

1. **Rust formatting**: Always use `cargo fmt` to format your code before committing.
2. **Clippy**: Ensure that your code is free of warnings by running `cargo clippy` before submitting a pull request.
3. **Comments and Documentation**: Use clear, concise comments to explain the logic of complex code blocks. All public APIs should have appropriate RustDoc comments.

## Commit Message Guidelines

We follow the **[Conventional Commits](https://www.conventionalcommits.org/)** specification to ensure a clear and consistent history.

### Format:

```
<type>(<scope>): <description>
```

- **type**: The type of the change, such as `feat` (feature), `fix` (bug fix), `docs` (documentation), `style` (formatting), `refactor` (refactoring), `test` (adding tests), etc.
- **scope**: The part of the codebase affected (optional).
- **description**: A short description of the change.

### Example Commit Messages:

```
feat(button): add new color variant for buttons
fix(navbar): resolve issue with dropdown not closing
docs(readme): update installation instructions
```

## Pull Request Guidelines

1. **Open a Pull Request**: Ensure your branch is up to date with `develop` and create a pull request (PR) to merge your branch into `develop`.
2. **Describe your changes**: Provide a clear description of the problem your PR solves and link to any related issues.
3. **Tests and Documentation**: Ensure all relevant tests are included, and update the documentation if necessary.
4. **CI Checks**: Your pull request must pass all CI checks (linting, formatting, tests) before being considered for review.

## Contribution Checklist

Before submitting your pull request, please ensure you have completed the following:

- [ ] Code is formatted using `cargo fmt`.
- [ ] Code passes all linting checks (`cargo clippy -- -D warnings`).
- [ ] All existing and new tests pass (`cargo test`).
- [ ] Commit messages follow the [conventional commits](https://www.conventionalcommits.org/) format.
- [ ] Relevant documentation has been updated or added.

## Issue Reporting and Discussion

If you find a bug or have an idea for a new feature, feel free to open an issue. Please include:

1. A clear and concise description of the problem or feature.
2. Steps to reproduce the problem (if it's a bug).
3. Screenshots or code snippets.
4. Label the issue appropriately (e.g., `bug`, `enhancement`, etc.).

We encourage open discussion and collaboration on all issues. Please be respectful and constructive in your comments.

## License

By contributing to this repository, you agree that your contributions will be licensed under the [MIT License](./LICENSE).

## Contributor Expectations

1. **Respectful Communication**: Always be respectful and constructive in discussions, reviews, and comments. We foster a collaborative and inclusive environment.
2. **Clear Documentation**: Ensure that all code changes are well-documented so that new contributors can easily understand and contribute to the project.
3. **Acknowledgment of Contributions**: We value every contribution, whether it's code, design, or feedback. All contributors will be credited in the project's releases.
