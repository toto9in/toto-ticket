# CLAUDE.md

`toto-ticket-backend` is the backend of a ticketing platform, written in Rust (edition 2024).

## Commands

```bash
cargo run                       # run the binary
cp .env.example .env            # DATABASE_URL for the app and integration tests
docker compose up -d --wait db  # database for integration tests
cargo test                      # run tests
cargo fmt                       # format
cargo clippy --all-targets      # lint
```

Before finishing a change: `cargo fmt --check && cargo clippy --all-targets && cargo test`.

## Agent skills

### Issue tracker

GitHub Issues on `toto9in/toto-ticket`, tracked on the `toto-ticket` GitHub Project board. See `docs/agents/issue-tracker.md`.

### Triage labels

Default five-role vocabulary (`needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`). See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: `CONTEXT.md` + `docs/adr/` at the repo root. See `docs/agents/domain.md`.
