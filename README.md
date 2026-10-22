# Munch Shell (msh)

Simple shell for linux. Interactive, bash-like, currently early v1.

## Status

Lexer + parser complete and tested; executor not built yet.
See `Goal/shell-v1.prot.md` for the v1 scope.

## Build & Run

```sh
cargo build
cargo run --bin msh        # interactive shell
cargo run --bin msh -- --help
```

## Tests

```sh
cargo test                      # all tests
cargo test --test LexerTest     # lexer only
cargo test --test ParserTest    # parser only
cargo test QuotedSpan           # tests matching a name
cargo test -- --nocapture       # show println! output
```

## Debug Tools

```sh
cargo run -q --bin ttylexdump -- 'ls -l | grep rs > out.txt'
cargo run -q --bin ttyparsedump -- 'ls -l | grep rs > out.txt'
```

`ttylexdump` prints the token stream; `ttyparsedump` prints the parsed AST
(or the syntax error). Both also read lines from stdin.

## License

Apache-2.0. See `LICENSE`.

Report issues at: <https://github.com/TwilightUnix/msh/issues>
