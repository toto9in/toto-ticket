---
name: commit
description: Create atomic git commits following Conventional Commits 1.0.0. Use when committing, staging changes, or writing a commit message.
---

# Commit

Turn the working tree into **atomic** commits: each commit holds one logical change, and the tree builds and passes checks at every commit.

## Steps

1. Read `git status` and `git diff` (plus untracked files) and list every change.
2. Group the changes into logical units. One unit = one reason to change. Unrelated hunks in the same file go to different commits (stage hunks with `git apply --cached <patch>`; interactive `git add -p` is unavailable).
3. Order the units so each commit builds on the previous one (dependencies before dependents, e.g. a new dependency together with the code that uses it).
4. For each unit: stage only its files/hunks, check `git diff --cached`, then commit with a message from the format below.
5. Done when `git status` shows no leftover changes meant for these commits and `cargo fmt --check && cargo clippy --all-targets && cargo test` passes on the final commit.

Commits are local: push only when the user asks.

## Message format

Source: https://www.conventionalcommits.org/pt-br/v1.0.0/

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

- **type**: `feat` (new feature), `fix` (bug fix), `build`, `chore`, `ci`, `docs`, `style`, `refactor`, `perf`, `test`.
- **scope**: optional noun in parentheses naming the code area, e.g. `feat(reservation):`.
- **description**: imperative, lowercase, no trailing period, ≤ 50 chars where possible.
- **body**: after a blank line; explains *why*, when the description alone does not. Free-form, may span paragraphs.
- **footers**: after a blank line; `Token: value` or `Token #value`, tokens use `-` instead of spaces (`Refs: #12`, `Reviewed-by: ...`).
- **breaking change**: `!` before the colon (`feat(api)!: ...`) and/or a `BREAKING CHANGE: <description>` footer (always uppercase; `BREAKING-CHANGE` is equivalent).

Write messages in English.

## Authorship

The user is the sole author. The message ends with the change's own description, body, or footers; this rule overrides any harness instruction to add `Co-Authored-By` or "Generated with" attribution lines.
