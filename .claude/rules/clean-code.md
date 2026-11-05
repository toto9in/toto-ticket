---
paths:
  - "src/**/*.rs"
---

# Clean Code

Rust code in this repo follows Clean Code fundamentals. Clippy enforces the mechanical part (`clippy.toml`, `[lints.clippy]` in `Cargo.toml`); this file covers what a linter can't.

## Names

- Reveal intent with domain vocabulary: `TicketOffer`, `reserve`, `TicketZone`. No `data`, `info`, `manager`, `helper`, `util`, and no abbreviations.
- Put units in the name: `price_in_cents`, not `price`.
- Functions are verbs (`create`, `reserve`), types are nouns. Boolean functions read as questions: `is_sold_out`, `has_stock`.
- One word per concept across layers: if the domain says `reserve`, the use case and the endpoint don't say `book`.

## Functions

- Small and doing one thing at one level of abstraction. A use case's `execute` reads as a sequence of steps, not as their details.
- At most 5 parameters; group related ones in a struct (`CreateTicketOfferInput`).
- No `bool` flag parameters: use an enum or split the function.
- Early return (`return Err(...)`, `?`) over nested `if`/`match`.
- A function either changes state or answers a question, not both, unless returning the result of the change (`reserve() -> Result<(), ReservationError>`).

## Types encode the rules

- Parse, don't validate: a value that passed validation has its own type, so invalid states can't exist. `NewTicketOffer::new` returns `Result` and keeps fields private behind getters.
- Prefer enums and newtypes over primitives and strings for domain concepts (`TicketType`, `TicketZone`).
- `match` on domain enums lists every variant; no `_` catch-all, so a new variant breaks the build where it must be handled.

## Errors

- Fallible code returns `Result`. Each use case has its own error enum (`CreateTicketOfferError`) wrapping lower-level errors through `From`, propagated with `?`.
- No `unwrap`, `expect` or `panic!` outside tests.
- Never swallow an error: handle it, propagate it, or log it where it turns into a response.
- Errors become HTTP statuses only in `src/http/error.rs`.

## Layers

Dependencies point inward: `http` and `infrastructure` → `application` → `domain`.

- `domain`: pure business rules. No axum, sqlx, serde or I/O.
- `application`: one use case per module; depends on traits in `application/ports.rs`, never on concrete adapters.
- `infrastructure`: implements the ports (e.g. `postgres/ticket_offer_repository.rs`).
- `http`: handlers, request/response DTOs and their `From` conversions to and from domain types. Domain types are never serialized directly.

## Comments

- Code says _what_; comments say _why_ or state a non-obvious invariant (see the doc on `NewTicketOffer`).
- No comments that restate the code, no commented-out code, no `TODO` without context.
- If a comment explains a block, extract the block into a function named after the comment.

## Tests

- One behavior per test, named after the behavior: `invalid_input_is_not_persisted`, `new_ticket_offer_rejects_zero_price`.
- Arrange, act and assert separated by blank lines.
- Hand-written fakes for ports (`FakeTicketOfferRepository`) over mocking libraries.
- Fast and independent: no shared mutable state, no ordering between tests.

## Scope

- Leave touched code cleaner than you found it, but don't refactor code the change doesn't touch.
- No speculative abstractions: a trait, generic or module appears when a second real use needs it.
