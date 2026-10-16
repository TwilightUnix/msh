# Munch Shell (msh)

Simple shell for linux. Interactive, bash-like, currently early v1.

## Status

Lexer + parser skeleton compile; executor not built yet.
See `Goal/shell-v1.prot.md` for the v1 scope.

## Build & Run

```sh
cargo build
cargo run --bin msh        # interactive shell
cargo run --bin msh -- --help
```

## Debug Tools

```sh
cargo run -q --bin ttylexdump -- 'ls -l | grep rs > out.txt'
```

Tokenizes one line (or stdin) and dumps the token stream.

## License

Apache-2.0. See `LICENSE`.

Report issues at: <https://github.com/TwilightUnix/msh/issues>
