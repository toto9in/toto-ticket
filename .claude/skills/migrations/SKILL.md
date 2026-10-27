---
name: migrations
description: Create and apply sqlx database migrations. Use when adding or changing tables, columns, indexes or constraints, or when a schema change is needed.
---

# Migrations

Schema changes ship as sqlx migrations in `migrations/`. The app applies pending ones on startup through `sqlx::migrate!()`; `build.rs` makes cargo recompile when that folder changes.

## Steps

1. Start the database: `docker compose up -d --wait` (Postgres on port 5433). `DATABASE_URL` comes from `.env`; when it is missing, ask the user to create it from `.env.example`.
2. Create the file: `sqlx migrate add <verb>_<object>` (snake_case, e.g. `create_ticket_offers`, `add_status_to_orders`). Migrations are up-only: one `.sql` file, no `-r`.
3. Write the SQL following the conventions below.
4. Apply and check: `sqlx migrate run`, then `sqlx migrate info` shows the new migration as `installed`.
5. Run `cargo build` so the binary embeds the new file, then the project checks from `CLAUDE.md`.

Done when the migration is `installed` on the local database and the checks pass.

## Conventions

- Tables: plural snake_case (`ticket_offers`, `reservations`).
- Ids: `BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY` (maps to the domain's `u64` ids via `i64`).
- Money: integer cents, `price_in_cents INTEGER NOT NULL`, mirroring the domain.
- Timestamps: `TIMESTAMPTZ` (the domain uses `DateTime<Utc>`).
- Domain enums (`ReservationStatus`, `OrderStatus`, ...): Postgres enum, `CREATE TYPE reservation_status AS ENUM (...)`, snake_case type and values. New values go in a later migration with `ALTER TYPE ... ADD VALUE`.
- Invariants the domain relies on go in the schema too, e.g. `CHECK (quantity >= 0)`.

## Changing the schema later

A migration is frozen once committed: sqlx stores each file's checksum and refuses to start when an applied file changes. Every fix or change is a new migration.

While a migration exists only on your machine and is still uncommitted, you may edit it after resetting the local database: `docker compose down -v`, then `docker compose up -d --wait` and `sqlx migrate run`.
