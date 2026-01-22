# Constitution

## Preamble

A personal Rust CLI tool built for long-term use with pragmatic quality standards. Ship fast, fix what hurts, and leverage Rust's type system to catch errors at compile time.

## Core Principles

### I. Ship Fast, Fix What Hurts

**Build the smallest useful thing, dogfood immediately, iterate on real pain.**

- **Rule 1**: No speculative features - build when needed, not "just in case"
- **Rule 2**: If it works and you're the only user, ship it
- **Rule 3**: Refactor when the pain is real, not when it's theoretical
- **Rule 4**: Perfect is the enemy of done

**Rationale**: As a solo developer building for yourself, you have the luxury of fast iteration. Real usage reveals real problems better than upfront planning.

### II. Unix Philosophy

**Single purpose. Text I/O. Composable. Predictable exit codes.**

- **Rule 1**: Do one thing well per command/subcommand
- **Rule 2**: Accept input from stdin, emit to stdout, errors to stderr
- **Rule 3**: Exit 0 on success, non-zero on failure (use distinct codes for different failures)
- **Rule 4**: Play well with pipes and shell scripts

**Rationale**: CLI tools that follow Unix conventions compose with the rest of the ecosystem. Future-you will thank present-you when scripting.

### III. Fail Fast & Loud

**Crash early with clear context. No silent failures.**

- **Rule 1**: Use Rust's `?` operator and `Result` types - don't swallow errors
- **Rule 2**: Include context in errors: what failed, why, and what to try
- **Rule 3**: Validate inputs at the boundary, fail immediately on bad data
- **Rule 4**: Use `anyhow` or `thiserror` for ergonomic error handling

**Rationale**: Rust makes error handling explicit. Embrace it. A clear crash beats silent corruption every time.

### IV. Let the Compiler Work

**Leverage Rust's type system to eliminate runtime bugs at compile time.**

- **Rule 1**: Make illegal states unrepresentable with enums and newtypes
- **Rule 2**: Prefer `Option` and `Result` over panics or sentinel values
- **Rule 3**: Use `#[must_use]` on functions with important return values
- **Rule 4**: If it compiles and the types are right, it's probably correct

**Rationale**: Rust's compiler is your test suite for a huge class of bugs. Invest in types, not tests, for structural correctness.

### V. Test What Matters

**Focus on catching bugs, not coverage metrics.**

- **Rule 1**: Test critical paths and complex logic
- **Rule 2**: Integration tests over unit tests for CLI behavior
- **Rule 3**: Skip testing trivial code the compiler already validates
- **Rule 4**: A test that never fails isn't testing anything

**Rationale**: You're testing critical paths only. Spend test budget where bugs actually hide, not where the type system already protects you.

### VI. Good Enough Architecture

**Use patterns you know. Boring and fast beats novel and slow.**

- **Rule 1**: Start with the simplest structure that works
- **Rule 2**: Extract modules when files get unwieldy, not before
- **Rule 3**: No design patterns for their own sake
- **Rule 4**: Complexity requires explicit justification

**Rationale**: This is a personal tool with indefinite lifespan. Keep it simple enough that future-you can jump back in after months away.

## Development Standards

### Error Messages

- Include the operation that failed: "Failed to read config file"
- Include the reason: "permission denied"
- Include remediation when possible: "Check file permissions or run with elevated privileges"

### CLI Interface

- Use `clap` for argument parsing with derive macros
- Provide `--help` and `--version` flags
- Support both short (`-v`) and long (`--verbose`) flags for common options
- Use subcommands for distinct operations

### Code Organization

- `main.rs` handles CLI parsing and orchestration
- Business logic in library code (`lib.rs` or modules)
- Keep functions small enough to fit on one screen

## Governance

### Amendment Procedure

- Changes require documented rationale
- Version follows semver (MAJOR.MINOR.PATCH)
- MAJOR: Breaking principle changes
- MINOR: New principles/sections
- PATCH: Clarifications

### Compliance

- Constitution supersedes other practices
- Complexity requires explicit justification
- When in doubt, ship it and iterate
