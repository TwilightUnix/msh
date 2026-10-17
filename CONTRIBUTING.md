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
PascalCase, same as types:

```rust
const MaxLine: usize = 4096;
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

Consistency over familiarity. The project intentionally avoids
`snake_case`/`SCREAMING_CASE` so the source reads uniformly.
`non_snake_case` and `non_camel_case_types` lints are allowed at the
crate root for this reason.

## Build

```sh
cargo build
cargo run --bin msh
cargo test
```
