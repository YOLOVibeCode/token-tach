# Contributing to Token Tach

## Dev setup

```bash
# Prerequisites: Rust (stable), Node 22, pnpm 10
nvm use                              # Node 22 (from .nvmrc)
corepack enable                      # Enable pnpm
pnpm install --frozen-lockfile       # Install deps
pnpm tauri dev                       # Launch app with HMR
```

## Stack

- **Frontend:** Svelte 5 (runes: `$state`, `$derived`, `$effect`) + TypeScript (strict)
- **Backend:** Rust (edition 2021) + Tauri v2
- **Storage:** SQLite via rusqlite
- **Credentials:** OS keychain via keyring crate
- **Tests:** vitest (frontend), cargo test (backend)

## Development rules

### TDD is mandatory

Every feature follows Red-Green-Refactor:
1. Write a failing test first
2. Write minimum code to pass
3. Refactor while tests stay green

No exceptions. See `CLAUDE.md` for the full rules.

### Conventional Commits

Every commit on `main` must follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add Cursor provider adapter
fix: handle rate limit retry-after header
test: add burn rate calculator edge cases
```

### Pre-flight

Run before every push:

```bash
pnpm preflight
```

This runs: `typecheck → vitest → cargo fmt --check → cargo clippy → cargo test`.

## Architecture

```
src-tauri/src/          # Rust backend
  providers/            # One module per AI tool (trait implementations)
  polling/              # Periodic fetch engine
  storage/              # SQLite behind Repository trait
  keychain/             # OS keychain behind trait
  commands/             # Tauri commands (thin dispatch)
  models/               # Shared types
  errors/               # Typed error hierarchy

src/lib/                # Svelte 5 frontend
  api/                  # Typed Tauri invoke wrappers
  components/           # UI components
  stores/               # Svelte 5 rune-based state
  types/                # TypeScript interfaces
```

## Code standards

- **Rust:** No `unwrap()` in production. Use `?` operator. `thiserror` for errors. `cargo clippy -- -D warnings`.
- **TypeScript:** `strict: true`. Zero `any` types.
- **Svelte:** Svelte 5 runes only. Components under 150 lines.
- **Security:** API keys in OS keychain only. Never log secrets.
- **Traits:** Small and focused (ISP). Max 5 methods per trait.
