# CLAUDE.md - Aider-Script Development Guide

## Commands

- Build: `cargo build`
- Run: `cargo run`
- Lint: `cargo clippy --fix --allow-dirty`
- Test all: `cargo test`
- Test specific: `cargo test test_name`
- Test module: `cargo test module_name`

## Code Style

- **Error Handling**: Use `anyhow` for error propagation with `?` operator
- **Types**: Prefer strong typing with generics and lifetime annotations
- **Naming**: Use snake_case for variables/functions, CamelCase for types
- **Modules**: Keep modules focused on single responsibility
- **Documentation**: Use doc comments (`///`) for public functions
- **Imports**: Group standard library, external crates, then local modules
- **Formatting**: Use rustfmt defaults

## Development Practices

- All code changes should include tests
- Be terse in documentation and responses
- No high-level explanations in comments
- Follow Rust idioms for pattern matching and Result handling
