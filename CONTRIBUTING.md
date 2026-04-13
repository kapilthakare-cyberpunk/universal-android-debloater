# Contributing to Universal Android Debloater

Thank you for your interest in contributing to UAD! This is a community project
and every contribution is welcome and appreciated.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Contributions](#making-contributions)
- [Adding Packages to Debloat Lists](#adding-packages-to-debloat-lists)
- [Code Style](#code-style)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Communication](#communication)

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/universal-android-debloater.git`
3. Create a branch for your feature: `git checkout -b feature/my-feature`

## Development Setup

### Prerequisites

- **Rust** (latest stable): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Android Platform Tools** (ADB): Required for testing device connectivity
- **Git**

### Building from Source

```bash
# Install dependencies
cargo build

# Build with default features (wgpu + self-update)
cargo build --release

# Build with OpenGL support (for older hardware)
cargo build --release --no-default-features --features glow,self-update

# Build without self-update (for package managers)
cargo build --release --no-default-features --features wgpu,no-self-update
```

### Running

```bash
cargo run
```

## Making Contributions

### Types of Contributions

1. **Package List Updates**: Add/update/remove packages from `resources/assets/uad_lists.json`
2. **Bug Fixes**: Fix crashes, UI issues, or ADB command problems
3. **New Features**: Add new functionality to the GUI or core
4. **Documentation**: Improve README, wiki, or code comments
5. **Performance**: Optimize rendering, ADB commands, or memory usage

### Adding Packages to Debloat Lists

The debloat list is located at `resources/assets/uad_lists.json`. Each package entry should have:

```json
{
  "id": "com.example.package",
  "list": "oem",
  "description": "Clear description of what this package does",
  "dependencies": [],
  "neededBy": [],
  "labels": [],
  "removal": "recommended"
}
```

**Removal categories:**
- `recommended`: Safe to remove for most users
- `advanced`: May affect some functionality
- `expert`: Will affect significant functionality
- `unsafe`: Known to cause bootloops or serious issues
- `unlisted`: Not categorized yet

**List categories:**
- `aosp`: Android Open Source Project packages
- `carrier`: Mobile carrier bloatware
- `google`: Google services
- `misc`: Miscellaneous packages
- `oem`: Manufacturer-specific packages
- `pending`: Awaiting review

## Code Style

We follow standard Rust conventions enforced by `rustfmt` and `clippy`:

```bash
# Format code
cargo fmt

# Run clippy (lint)
cargo clippy --all-features -- -D warnings

# Check without building
cargo check --all-features
```

### Guidelines

- Use descriptive variable names
- Document public APIs with `///` doc comments
- Handle errors gracefully (no `.unwrap()` in production code)
- Use `Result` and `Option` types appropriately
- Keep functions focused and reasonably sized

## Testing

Run the test suite before submitting a PR:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_package_state_display
```

### Adding Tests

- Place unit tests in `#[cfg(test)] mod tests` blocks within the source file
- Tests should be fast and not require ADB or network access
- Test edge cases and error conditions

## Pull Request Process

1. **Commit messages**: Follow [Conventional Commits](https://www.conventionalcommits.org/)
   - `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`
2. **Update CHANGELOG.md** if your change is user-facing
3. **Ensure CI passes**: All checks must be green
4. **Request review**: Tag a maintainer or wait for automatic assignment
5. **Address feedback**: Be responsive to review comments

### PR Checklist

- [ ] Code compiles without warnings (`cargo check --all-features`)
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Clippy passes (`cargo clippy --all-features -- -D warnings`)
- [ ] CHANGELOG.md updated (if applicable)
- [ ] Documentation updated (if applicable)

## Communication

- **Issues**: For bug reports and feature requests
- **Discussions**: For questions and general discussion
- **Wiki**: For documentation and guides

## License

By contributing, you agree that your contributions will be licensed under the
GPL-3.0 License.
