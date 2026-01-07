# Contributing to List Big Files

Thank you for your interest in contributing to List Big Files! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When creating a bug report:

1. Use a clear and descriptive title
2. Provide a detailed description of the problem
3. Include steps to reproduce the issue
4. Provide expected vs actual behavior
5. Include your operating system and Rust version
6. If applicable, include screenshots or error messages

### Suggesting Enhancements

Enhancement suggestions are welcome! When suggesting enhancements:

1. Use a clear and descriptive title
2. Provide a detailed description of the proposed enhancement
3. Explain why this enhancement would be useful
4. Provide examples of how the enhancement would work
5. List any possible drawbacks or alternative approaches

### Pull Requests

Pull requests are the best way to propose changes. Here's how to submit one:

1. Fork the repository
2. Create a new branch for your feature or bugfix
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bugfix-name
   ```
3. Make your changes
4. Write/update tests if applicable
5. Ensure the code follows the project's style guidelines
6. Commit your changes with clear, descriptive messages
7. Push to your fork
   ```bash
   git push origin feature/your-feature-name
   ```
8. Create a pull request with a clear description

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Setting Up

```bash
# Fork and clone your fork
git clone https://github.com/yourusername/list_big_files.git
cd list_big_files

# Add the upstream repository
git remote add upstream https://github.com/original-owner/list_big_files.git

# Build the project
cargo build

# Run tests
cargo test
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific tests
cargo test test_name

# Run tests in release mode
cargo test --release
```

### Building

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release

# Run the program
cargo run -- --help
```

### Code Style

This project uses standard Rust formatting:

```bash
# Format code
cargo fmt

# Check formatting without making changes
cargo fmt --check
```

### Linting

```bash
# Run clippy linter
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

## Project Structure

```
list_big_files/
├── src/
│   └── main.rs          # Main source code
├── Cargo.toml            # Project metadata and dependencies
├── Cargo.lock            # Dependency lock file
├── README.md             # Project documentation
├── LICENSE               # MIT License
├── .gitignore            # Git ignore patterns
├── CONTRIBUTING.md       # This file
└── build.sh              # Build script
```

## Coding Guidelines

### Rust Best Practices

- Use `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Write clear, concise variable and function names
- Add comments for complex logic
- Use error handling with `Result` types

### Code Comments

- Add comments for complex algorithms
- Document public APIs with `///` doc comments
- Use `//` for inline comments when necessary
- Keep comments up-to-date with code changes

### Commit Messages

Follow these guidelines for commit messages:

- Use the imperative mood ("Add feature" not "Added feature")
- Keep the first line under 50 characters
- Reference issue numbers when applicable
- Separate subject from body with a blank line
- Wrap body text at 72 characters

Examples:
```
feat: Add support for GB size units

Implements the ability to specify file sizes in gigabytes
using the 'GB' or 'G' suffix. This resolves #123.

Closes #123
```

```
fix: Handle directory permission errors gracefully

Updates error handling to skip directories that cannot be
accessed rather than failing the entire scan.
```

## Testing

### Writing Tests

- Write tests for new features
- Test both success and failure cases
- Use descriptive test names
- Keep tests fast and independent

### Test Coverage

While we don't have a specific coverage requirement, strive for good test coverage:

- Test public functions and APIs
- Test edge cases and error conditions
- Test with realistic data

## Release Process

Releases are managed by the project maintainers:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Tag the release
4. Create GitHub release
5. Publish to crates.io (if applicable)

## Getting Help

If you need help:

- Check existing documentation
- Search existing issues and discussions
- Create a new issue with the "question" label
- Join community discussions (if available)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors who provide significant contributions will be recognized in the project's documentation.

Thank you for contributing to List Big Files!
