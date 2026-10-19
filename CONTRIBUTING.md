# Contributing to msh

First of all thank you for deciding to contribute to this project. Here are some
rules and standards for contributing to this project.

## Naming convention

This codebase uses a deliberate C-style naming scheme, not standard Rust
style. Match it in all new code.

| What | Rule | Example |
| --- | --- | --- |
| Files, structs, classes, enums | PascalCase | `Lexer.rs`, `Pipeline` |
| Functions, methods | PascalCase | `Tokenize`, `ParseList` |
| Namespaces, modules | PascalCase | `TTY::Lexer`, `mod Parser;` |
| Variables, members, params | snake_case | `tklist`, `redirects` |
| Constants | UPPER_SNAKE | `DEFAULT_PROMPT` |
| Enum values | UPPERCASE_ prefix from the enum name | `TK_Word` (`TokenKind`) |

Enum prefixes are short and per enum: `TK_` (`TokenKind`), `SP_` (`Separator`),
`ERR_` (`Error`).

Modules and filenames match: `Source/TTY/Lexer.rs` -> `mod Lexer;`.

The `non_snake_case` and `non_camel_case_types` lints are allowed at the
crate root for this reason.

### Why

Consistency over familiarity. The project avoids `snake_case` for types
and functions so the source reads uniformly; constants are the exception
and use the conventional `UPPER_SNAKE_CASE`.

## License of contributions

msh is released under the Apache License 2.0 (see `LICENSE`). By submitting a
pull request you agree that your contribution is licensed under the same
terms, without any additional conditions.
