# Contributing to msh

## Naming Convention

This codebase uses a deliberate C-style naming scheme, not standard Rust style.
Match it in all new code.

### Types
PascalCase with no underscores:

```rust
struct SimpleCommand { ... }
enum TokenKind { ... }
pub struct Pipeline { ... }
```

### Enum variants
`PREFIX_PascalCase` — a short uppercase prefix for the group, then PascalCase:

```rust
enum TokenKind {
      TK_Word,
      TK_Pipe,
      TK_Semicolon,
}
enum Separator {
      SP_Semicolon,
      SP_Background,
}
```

`TK_` for tokens, `SP_` for separators. Pick a short prefix per enum.

### Functions
PascalCase, including `pub fn` and methods:

```rust
fn RunCommandLine() { ... }
pub fn ParseList(...) { ... }
pub fn Tokenize(...) { ... }
```

### Variables, fields, params
snake_case:

```rust
let mut token_list = ...;
let mut pos: usize = 0;
pub val: &'a str,
```

### Constants
UPPER_SNAKE_CASE:

```rust
const MAX_LINE: usize = 4096;
const DEFAULT_PROMPT: &str = "msh> ";
```

### Files and modules
PascalCase filenames matching the module name:

```
Source/TTY/Lexer.rs   ->  mod Lexer;
Source/TTY/Parser.rs  ->  mod Parser;
```

### Error variant prefix
`ERR_`:

```rust
enum Error {
      ERR_Unknown,
      ERR_Syntax(String),
      ERR_UnexpectedEOF,
}
```

## Why

Consistency over familiarity. The project avoids `snake_case` for types
and functions so the source reads uniformly; constants are the exception
and use the conventional `UPPER_SNAKE_CASE`. The `non_snake_case` and
`non_camel_case_types` lints are allowed at the crate root for this
reason.

## License of contributions

msh is released under the Apache License 2.0 (see `LICENSE`). By submitting a
pull request you agree that your contribution is licensed under the same
terms, without any additional conditions.

## Build

```sh
cargo build
cargo run --bin msh
cargo test
```
