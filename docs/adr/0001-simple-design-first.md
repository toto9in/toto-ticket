# Simple design first, overengineer later

Status: accepted

The backend is built in two phases. Phase 1 (now) ships features with the simplest design that works inside the existing layers (`domain`, `application`, `infrastructure`, `http`). Phase 2 deliberately overengineers it (e.g. CQRS, event sourcing, messaging, caching, split services) as a learning exercise.

Until phase 2 starts — the owner says so explicitly — don't introduce new architectural patterns, infrastructure or abstractions beyond what the current feature needs. An overengineering idea spotted during phase 1 becomes a GitHub issue labelled `overengineer`, not code.
