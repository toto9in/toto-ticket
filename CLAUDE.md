# CLAUDE.md

`toto-ticket-backend` is the backend of a ticketing platform, written in Rust (edition 2024).

## Commands

```bash
cargo run                   # run the binary
cargo test                  # run tests
cargo fmt                   # format
cargo clippy --all-targets  # lint
```

Before finishing a change: `cargo fmt --check && cargo clippy --all-targets && cargo test`.
