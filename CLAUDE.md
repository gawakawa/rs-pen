# CLAUDE.md

## Commands

- `nix fmt` - Format code
- `nix flake check` - Run checks (format, lint)
- `nix build` - Build the project

## TDD

- Use TDD (Test-Driven Development) when writing code:
  1. **Red** - Write a failing test, run `nix flake check` to see it fail
  2. **Green** - Write the minimum code, run `nix flake check` to see it pass
  3. **Refactor** - Clean up the code while keeping tests green
