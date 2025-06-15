# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`rc` is a terminal calculator written in Rust, designed for scientific/engineering calculations. It supports complex numbers, units, user-defined variables and functions, and features a REPL with line editing and history.

## Development Commands

### Build and Run
```bash
cargo build                    # Build debug version
cargo build --release         # Build optimized version
cargo run                     # Run in REPL mode
cargo run -- [expression]     # Evaluate expression directly
```

### Testing
```bash
cargo test                     # Run all tests (unit tests + integration tests)
cargo run -- --test          # Run built-in tests
```

### Code Quality
```bash
cargo fmt                     # Format code
cargo clippy                  # Run linter
```

### Build Cross-Platform Binaries
```bash
# Linux (static linking with MUSL)
rustup target add x86_64-unknown-linux-musl
cargo build --release --target=x86_64-unknown-linux-musl

# Windows static linking configured in .cargo/config
cargo build --release --target=x86_64-pc-windows-msvc
```

### Additional Build Options
```bash
cargo install --path .                    # Install to ~/.cargo/bin/rc
cargo run -- -h                          # Show help
cargo run -- -v                          # Show version (includes git commit hash)
cargo run -- -d                          # Debug mode
cargo run -- -i [init_file]              # Use custom init file
cargo run -- -s [script_file]            # Run script file
```

## Architecture

The calculator follows a classic 3-layer architecture:

1. **Lexer** (`src/lexer.rs`) - Tokenizes input strings into `Token` enum variants
2. **Parser** (`src/parser.rs`) - Converts tokens into AST (`Node` enum) using recursive descent parsing  
3. **Evaluator** (`src/lib.rs` eval functions) - Recursively evaluates AST nodes

### Core Data Structures

- `Token` enum - Represents lexical tokens (numbers, operators, identifiers)
- `Node` enum - AST nodes supporting integers, floats, complex numbers, and units
- `Env` struct - Environment holding constants, variables, functions, and settings

### Key Features Implementation

- **Complex Numbers**: Uses `num-complex` crate, with automatic type promotion (int → float → complex)
- **Units**: Stored as AST in number nodes, with conversion and arithmetic support  
- **User Functions**: Use `_1`..`_9` as parameter placeholders, implemented via token substitution
- **REPL**: Terminal control via `crossterm` crate with bracket highlighting

### File Organization

- `src/main.rs` - CLI argument parsing and application entry point
- `src/lib.rs` - Core evaluation logic and error types
- `src/env.rs` - Environment management (constants, variables, functions)
- `src/lexer.rs` - Tokenization logic
- `src/parser.rs` - AST construction  
- `src/readline.rs` - REPL and terminal interaction
- `src/script.rs` - Script execution mode
- `src/units.rs` - Unit system implementation
- `src/run_test.rs` - Built-in test suite

## Important Notes

- The lexer splits `-100` into unary `-` and `100` for easier parsing
- Error handling uses `MyError` enum with `thiserror` derive macro
- Version info automatically includes git commit hash via `build.rs`
- User-defined functions limited to 9 parameters (`_1` through `_9`)
- Cross-platform terminal support prioritizes Windows compatibility
- Unit tests are comprehensive and should be run after any changes

## Testing Strategy

The project uses multiple testing approaches:
- Unit tests in each module (`#[cfg(test)]`)
- Integration tests in `tests/` directory  
- Built-in test suite accessible via `--test` flag
- Script-based regression tests comparing expected outputs

Always run the full test suite (`cargo test`) before committing changes.