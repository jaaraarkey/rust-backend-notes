# Contributing to Notes App Backend

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone <your-fork-url>`
3. Create a feature branch: `git checkout -b feature/amazing-feature`
4. Make your changes
5. Test your changes: `cargo test`
6. Format code: `cargo fmt`
7. Check with clippy: `cargo clippy`
8. Commit changes: `git commit -m 'Add amazing feature'`
9. Push to branch: `git push origin feature/amazing-feature`
10. Open a Pull Request

## Code Standards

### Rust Style Guide
- Follow official Rust formatting with `cargo fmt`
- Use `cargo clippy` and fix all warnings
- Write tests for new functionality
- Document public APIs with doc comments

### GraphQL Schema Guidelines
- Use clear, descriptive field names
- Provide field descriptions in schema
- Follow GraphQL naming conventions (camelCase)
- Handle errors gracefully with proper error types

### Commit Messages
- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit first line to 72 characters
- Reference issues and pull requests liberally

## Development Workflow

### Setting up Development Environment
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install rust-analyzer for your editor
# VS Code: Install rust-analyzer extension
# Vim/Neovim: Configure with your LSP client

# Clone and setup project
git clone <repo-url>
cd backend
cargo build
cargo run
```

### Testing
```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test

# Run specific test
cargo test test_hello_query
```

### Code Quality Checks
```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy

# Security audit
cargo audit
```

## Pull Request Process

1. Update README.md with details of changes if needed
2. Update version numbers following [SemVer](http://semver.org/)
3. Ensure all tests pass
4. Ensure code is formatted and passes clippy
5. Get approval from maintainers

## Issue Reporting

When reporting issues, please include:
- Rust version (`rustc --version`)
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Relevant log output

## Feature Requests

We welcome feature requests! Please:
- Check existing issues first
- Clearly describe the feature
- Explain why it would be useful
- Consider implementation approaches