<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

**Table of Contents**

- [Contributing to Grimoire CSS](#contributing-to-grimoire-css)
  - [Development Workflow](#development-workflow)
    - [Branch Structure](#branch-structure)
    - [Workflow Overview](#workflow-overview)
      - [1. Proposing a New Feature, Fix, or Chore](#1-proposing-a-new-feature-fix-or-chore)
      - [2. Development](#2-development)
      - [3. Preparing a Release](#3-preparing-a-release)
      - [4. Finalizing a Release](#4-finalizing-a-release)
      - [5. Handling Hotfixes](#5-handling-hotfixes)
      - [6. Handling Chores](#6-handling-chores)
      - [7. Updating Feature, Refactor, and Fix Branches](#7-updating-feature-refactor-and-fix-branches)
    - [Automated Tagging and Publishing](#automated-tagging-and-publishing)
    - [Repository Security and Permissions](#repository-security-and-permissions)
    - [Project Management](#project-management)
    - [Contribution Opportunities](#contribution-opportunities)
  - [Setting Up the Project Locally](#setting-up-the-project-locally)
  - [Code Quality Standards](#code-quality-standards)
  - [Code Style Guidelines](#code-style-guidelines)
  - [Commit Message Guidelines](#commit-message-guidelines)
    - [Format:](#format)
    - [Example Commit Messages:](#example-commit-messages)
  - [Pull Request Guidelines](#pull-request-guidelines)
  - [Contribution Checklist](#contribution-checklist)
  - [Issue Reporting and Discussion](#issue-reporting-and-discussion)
    - [Issues](#issues)
    - [Discussions](#discussions)
  - [License](#license)
  - [Contributor Expectations](#contributor-expectations)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# Contributing to Grimoire CSS

Thank you for considering contributing to Grimoire CSS! We welcome all forms of contribution: code, documentation, examples, and more. This guide outlines the process and standards for contributing to the project.

## Development Workflow

We follow a structured Git workflow to manage the lifecycle of code and contributions. This approach provides organization while remaining flexible to accommodate contributions from the community.

### Branch Structure

- **`main`**: The stable branch where production-ready code resides. Only thoroughly tested and approved code is merged here via pull requests.

- **Feature Branches**: Branches for developing individual features or enhancements.

  - Naming convention: `feature/{feature-name}`

- **Fix Branches**: Branches for regular bug fixes intended for the next release.

  - Naming convention: `fix/{fix-description}`

- **Release Candidate Branches**: Branches for preparing specific releases.

  - Naming convention: `rc/{version}` (e.g., `rc/1.2.0`)

- **Hotfix Branches**: Branches for urgent fixes to production code that cannot wait for the next release.

  - Naming convention: `hotfix/{hotfix-description}`

- **Refactor Branches**: Branches for code refactoring that improves code structure, readability, or performance without changing functionality.

  - Naming convention: `refactor/{refactor-description}`

- **Chore Branches**: Branches for maintenance tasks such as updating documentation, CI configurations, or other non-feature/non-fix changes.
  - Naming convention: `chore/{chore-description}`

### Workflow Overview

#### 1. Proposing a New Feature, Fix, or Chore

We encourage contributors to propose new features, fixes, or chores, even if they're not part of our current roadmap or milestones.

- **Open an Issue**:
  - Before starting work, please [open an issue](https://github.com/persevie/grimoire-css/issues/new/choose) to propose your feature, describe the bug you're fixing, or explain the chore task.
  - Use the appropriate issue template provided to give detailed information.
  - This allows maintainers and the community to discuss your proposal and provide feedback.

#### 2. Development

- **Create a Branch from `main`**:

  - For features:
    ```bash
    git checkout -b feature/{feature-name} main
    ```
  - For regular fixes:
    ```bash
    git checkout -b fix/{fix-description} main
    ```
  - For refactoring:
    ```bash
    git checkout -b refactor/{refactor-description} main
    ```
  - For hotfixes:
    ```bash
    git checkout -b hotfix/{hotfix-description} main
    ```
  - For chores:
    ```bash
    git checkout -b chore/{chore-description} main
    ```

- **Implement Your Changes**:

  - Follow the code quality and style guidelines outlined below.
  - Include tests and documentation updates as necessary.

- **Open a Pull Request**:

  - **Target Branch**:
    - For features, refactoring, and regular fixes:
      - Open a PR against the appropriate `rc/{version}` branch.
      - If unsure which release your contribution will fit into, you can initially target your PR to `main`, and maintainers will retarget it as needed.
    - For hotfixes and chores:
      - Open a PR directly against `main`.
  - **PR Description**:
    - Reference the issue you opened (e.g., "Closes #123").
    - Provide a clear description of your changes.

- **Collaborate on the Review**:
  - Be responsive to feedback from maintainers and reviewers.
  - Discuss any questions or concerns raised during the review process.

#### 3. Preparing a Release

- **Creating a Release Candidate Branch**:

  - **Branch from `main`**:
    ```bash
    git checkout -b rc/{version} main
    ```
  - **Example**:
    ```bash
    git checkout -b rc/1.2.0 main
    ```

- **Merging Feature, Refactor, and Fix Branches**:

  - Maintainers will merge feature (`feature/**`), refactor (`refactor/**`), and fix (`fix/**`) branches intended for the release into the `rc/{version}` branch via pull requests.

- **Testing and Stabilization**:
  - Perform thorough testing on the `rc/{version}` branch.
  - Resolve any issues or bugs discovered during testing.
  - Update documentation and version numbers as needed.

#### 4. Finalizing a Release

- **Merge into `main` via Pull Request**:

  - Open a pull request from `rc/{version}` into `main`.
  - Ensure all CI checks pass and that the code is reviewed.
  - Once approved, merge the PR into `main`.

- **Automated Tagging and Publishing**:

  - When a release candidate branch is merged into `main`, our GitHub Actions workflow automatically creates a tag (e.g., `v1.2.0`) based on the `rc/{version}` branch name.
  - The CI/CD pipeline then builds, releases, and publishes the new version.
  - **Note**: Do not manually create or push tags; the CI/CD pipeline handles tagging.

- **Delete the Release Candidate Branch**:
  ```bash
  git branch -d rc/{version}
  git push origin --delete rc/{version}
  ```

#### 5. Handling Hotfixes

- **Creating a Hotfix Branch**:

  - **Branch from `main`**:
    ```bash
    git checkout -b hotfix/{hotfix-description} main
    ```

- **Implementing the Hotfix**:

  - Develop and commit the hotfix.
  - Ensure all tests pass and code quality standards are met.

- **Open a Pull Request to `main`**:

  - Open a PR from `hotfix/{hotfix-description}` to `main`.
  - Reference the related issue and provide a clear description.

- **Review and Merge**:

  - Ensure the PR passes all CI checks and undergoes code review.
  - Upon approval, merge the PR into `main`.

- **Automated Tagging and Publishing**:

  - When the hotfix PR is merged into `main`, the CI/CD pipeline automatically increments the patch version and creates a new tag (e.g., from `v1.2.0` to `v1.2.1`).
  - The pipeline then builds, releases, and publishes the hotfix.
  - **Note**: Do not manually create or push tags; the CI/CD pipeline handles tagging.

- **Updating Other Branches**:
  - **Merge the Hotfix into Active `rc/{version}` Branches**:
    - If there are active release candidate branches, merge `main` into them to ensure the fix is included in future releases.
    ```bash
    git checkout rc/{version}
    git merge main
    ```

#### 6. Handling Chores

- **Creating a Chore Branch**:

  - **Branch from `main`**:
    ```bash
    git checkout -b chore/{chore-description} main
    ```

- **Implementing the Chore**:

  - Make the necessary changes (e.g., updating documentation, CI configurations).

- **Open a Pull Request to `main`**:

  - Open a PR from `chore/{chore-description}` to `main`.
  - Provide a clear description of the changes.

- **Review and Merge**:

  - Ensure the PR passes all CI checks and undergoes code review.
  - Upon approval, merge the PR into `main`.

- **Note**:
  - Merging `chore/**` branches into `main` will **not** trigger tagging or a new release.

#### 7. Updating Feature, Refactor, and Fix Branches

- **Sync with `main`**:
  - Regularly merge `main` into your feature (`feature/**`), refactor (`refactor/**`), and fix (`fix/**`) branches to keep them up to date.
    ```bash
    git checkout feature/{feature-name}
    git merge main
    ```
  - Resolve any merge conflicts that may arise.

### Automated Tagging and Publishing

Our CI/CD pipeline automates the tagging and publishing process:

- **Automated Tagging**:

  - When a release candidate (`rc/{version}`) or hotfix (`hotfix/**`) branch is merged into `main`, the GitHub Actions workflow automatically creates a tag:
    - For releases: `v{version}` (e.g., `v1.2.0`)
    - For hotfixes: Increments the patch version (e.g., from `v1.2.0` to `v1.2.1`)
  - **Important**: Do not manually create or push tags; the CI/CD pipeline handles tagging.

- **Build and Release**:

  - The pipeline builds the project for multiple platforms and creates a GitHub release with the compiled artifacts.

- **Publishing**:
  - The release is automatically published to [crates.io](https://crates.io) and other package managers as appropriate.

### Repository Security and Permissions

- **Branch Protection**:

  - The `main` branch is protected to ensure code quality and security.
  - All changes must be submitted via pull requests and reviewed before merging.
  - Direct pushes to `main` are not allowed.

- **Tagging Policy**:
  - Only the CI/CD pipeline should create and push tags for releases.
  - Manual creation or pushing of tags is prohibited to prevent unintended releases.
  - If you believe a tag needs to be created or updated, please discuss it with the maintainers.

### Project Management

We use GitHub Projects to maintain development and planning. Our project management process includes:

- **Issues**:

  - All tasks, features, bugs, and discussions are tracked using [GitHub Issues](https://github.com/persevie/grimoire-css/issues).
  - Issues are labeled appropriately (e.g., `feature`, `bug`, `documentation`, `good first issue`, `chore`).
  - **Issue Templates**: We provide issue templates to guide you in providing the necessary information.

- **Milestones**:

  - Issues are organized into milestones corresponding to planned releases.
  - Milestones help track progress toward release goals.

- **Project Board**:

  - We maintain a [GitHub Project board](https://github.com/orgs/persevie/projects/4/views/1) that visualizes the progress of tasks across different stages (e.g., Backlog, In Progress, Review, Done).
  - The board provides transparency and helps coordinate work among contributors.

- **Roadmap**:
  - Our [roadmap](https://github.com/orgs/persevie/projects/4/views/4) is reflected in the milestones and issues, providing visibility into upcoming features and priorities.

### Contribution Opportunities

- **Find an Issue**:

  - Browse the [Issues](https://github.com/persevie/grimoire-css/issues) to find tasks labeled as `help wanted` or `good first issue`.
  - Comment on the issue to express your interest in working on it.

- **Propose a New Idea**:

  - If you have an idea that's not already listed, feel free to open a new issue using the appropriate template.
  - Engage in discussions to refine your proposal.

- **Use GitHub Discussions**:
  - For questions, ideas, or general discussions, please use [GitHub Discussions](https://github.com/persevie/grimoire-css/discussions) instead of creating an issue.
  - This helps keep issues focused on actionable tasks and fosters community engagement.

## Setting Up the Project Locally

To contribute to Grimoire CSS, you'll need to set up the project on your local machine. Follow these steps:

1. **Fork the Repository**:

   - Click the **"Fork"** button at the top-right corner of the repository page to create a copy under your GitHub account.

2. **Clone the Repository**:

   ```bash
   git clone git@github.com:persevie/grimoire-css.git
   cd grimoire-css
   ```

3. **Install Rust** (if you don't have it installed already):

   - Follow the official [Rust installation guide](https://www.rust-lang.org/tools/install).

4. **Build the Project**:

   ```bash
   cargo build
   ```

5. **Run the Tests** to make sure everything is set up correctly:

   ```bash
   cargo test
   ```

After this setup, you're ready to start contributing to the project.

## Code Quality Standards

Maintaining high-quality code is crucial for the stability and maintainability of the project. Please follow these guidelines:

1. **Code Formatting**:

   - All code should be formatted using `cargo fmt`.
   - Our CI pipeline ensures that improperly formatted code will fail the build.

2. **Linting**:

   - Use `cargo clippy` to ensure your code is free of warnings.
   - We treat warnings as errors:

     ```bash
     cargo clippy -- -D warnings
     ```

3. **Tests**:

   - All new features or fixes should include appropriate unit and/or integration tests.
   - Ensure tests pass before submitting a pull request:

     ```bash
     cargo test
     ```

## Code Style Guidelines

To maintain consistency across the project, please adhere to the following code style guidelines:

1. **Rust Formatting**:

   - Always use `cargo fmt` to format your code before committing.

2. **Clippy**:

   - Ensure that your code is free of warnings by running `cargo clippy` before submitting a pull request.

3. **Comments and Documentation**:

   - Use clear, concise comments to explain the logic of complex code blocks.
   - All public APIs should have appropriate RustDoc comments.

## Commit Message Guidelines

We follow the **[Conventional Commits](https://www.conventionalcommits.org/)** specification to ensure a clear and consistent history.

### Format:

```
<type>(<scope>): <description>
```

- **type**: The type of the change, such as:

  - `feat`: A new feature
  - `fix`: A bug fix
  - `docs`: Documentation only changes
  - `style`: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc.)
  - `refactor`: A code change that neither fixes a bug nor adds a feature
  - `perf`: A code change that improves performance
  - `test`: Adding missing tests or correcting existing tests
  - `chore`: Changes to the build process or auxiliary tools and libraries such as documentation generation
  - `hotfix`: A critical fix to production code

- **scope**: The part of the codebase affected (optional).

- **description**: A short description of the change.

### Example Commit Messages:

```
feat(animation): add new fade-in animation
fix(parser): resolve panic when input is empty
docs(contributing): update guidelines for new contributors
chore(ci): update GitHub Actions workflows
hotfix(auth): fix security vulnerability in authentication
```

## Pull Request Guidelines

1. **Stay Up-to-Date**:

   - Before starting your work, ensure your local `main` branch is up-to-date:

     ```bash
     git checkout main
     git pull upstream main
     ```

2. **Open a Pull Request**:

   - Ensure your branch is up to date with `main` and create a pull request (PR) to merge your branch into the appropriate branch.
   - **Target Branch**:
     - For features and fixes: Target the appropriate `rc/{version}` branch or `main` if unsure.
     - For hotfixes and chores: Target `main`.

3. **Describe Your Changes**:

   - Provide a clear description of the problem your PR solves.
   - Reference any related issues (e.g., "Closes #123").

4. **Use the Pull Request Template**:

   - Fill out the [Pull Request Template](./.github/PULL_REQUEST_TEMPLATE.md) to provide all necessary information.

5. **Tests and Documentation**:

   - Ensure all relevant tests are included.
   - Update the documentation if necessary.

6. **CI Checks**:

   - Your pull request must pass all CI checks (linting, formatting, tests) before being considered for review.

7. **Code Review**:

   - Be responsive to feedback and make necessary changes promptly.
   - Engage in constructive discussions to improve the code.

## Contribution Checklist

Before submitting your pull request, please ensure you have completed the following:

- [ ] Code is formatted using `cargo fmt`.
- [ ] Code passes all linting checks (`cargo clippy -- -D warnings`).
- [ ] All existing and new tests pass (`cargo test`).
- [ ] Commit messages follow the [Conventional Commits](https://www.conventionalcommits.org/) format.
- [ ] Relevant documentation has been updated or added.
- [ ] The pull request is targeted at the correct branch (`main` or `rc/{version}`).
- [ ] You have filled out the pull request template.

## Issue Reporting and Discussion

### Issues

Issues are used to track **bugs**, **enhancements**, and **tasks** that need attention.

- **When to Open an Issue**:

  - Reporting a bug or unexpected behavior.
  - Proposing a new feature or enhancement.
  - Noting a task or improvement needed in the project.

- **Using Issue Templates**:

  - We provide issue templates to guide you in providing the necessary information.
  - Select the appropriate template when creating a new issue:
    - **Bug Report**
    - **Feature Request**
    - **Question**

- **Guidelines for Issues**:
  - Provide detailed information to help us understand and reproduce the issue.
  - Be respectful and constructive in your descriptions and comments.
  - Use clear and descriptive titles.

### Discussions

For **questions**, **ideas**, or **general discussions** that are not directly related to a specific issue, please use [GitHub Discussions](https://github.com/persevie/grimoire-css/discussions).

- **When to Use Discussions**:

  - Asking for help or clarification on using Grimoire CSS.
  - Sharing ideas or suggestions that are not yet fully formed as issues.
  - Engaging with the community and maintainers in open-ended conversations.

- **Benefits of Using Discussions**:

  - Keeps issues focused on actionable tasks.
  - Fosters a collaborative and inclusive community.
  - Provides a space for knowledge sharing and collective problem-solving.

- **How to Participate**:
  - Browse existing discussions to see if your topic is already being discussed.
  - Start a new discussion if your topic is new.
  - Be respectful, courteous, and follow our community guidelines.

## License

By contributing to this repository, you agree that your contributions will be licensed under the [MIT License](./LICENSE).

## Contributor Expectations

1. **Respectful Communication**:

   - Always be respectful and constructive in discussions, reviews, and comments.
   - We foster a collaborative and inclusive environment.

2. **Clear Documentation**:

   - Ensure that all code changes are well-documented so that new contributors can easily understand and contribute to the project.

3. **Acknowledgment of Contributions**:

   - We value every contribution, whether it's code, design, or feedback.
   - All contributors will be credited in the project's releases.
