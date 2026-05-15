# CLAUDE.md — Token Tach Development Rules

## Project

Token Tach: a macOS menubar/dock utility that monitors AI coding tool spend in real time.
Three-layer UI: tray (two numbers), hover (per-provider breakdown), click (full translucent HUD).
Stack: Tauri v2 (Rust) + Svelte 5 (TypeScript) + SQLite + OS Keychain.

## The Prime Directive: TDD

Every line of production code is written to make a failing test pass. No exceptions.

### The Cycle

1. **RED** — Write a failing test first. The test defines the behavior. If you cannot articulate the test, you do not understand the requirement.
2. **GREEN** — Write the minimum code to pass. Nothing more. No "while I'm here" additions.
3. **REFACTOR** — Improve structure, naming, duplication. Tests stay green. No behavior changes.

### Test Rules

- Write the test file before the implementation file. If both are new, the test file is created first.
- Rust tests: `#[cfg(test)] mod tests` in-file for unit tests. `tests/` directory for integration tests.
- Svelte/TS tests: colocated `<name>.test.ts` files. Use vitest + @testing-library/svelte.
- E2E tests: `tests/e2e/` directory.
- Every public function, every Tauri command, every Svelte component, every provider adapter — tested.
- Test names: `test_<unit>_<scenario>_<expected>` (Rust) or `describe/it` blocks (TS).
- Tests must be independent, deterministic, fast. No sleep waits, no shared state, no network calls in unit tests.
- Mock external APIs with realistic fixture data in `tests/fixtures/`.
- Coverage: 80% minimum, 90%+ for providers and core logic.

### When I Ask You to Build Something

Do this:
1. Create the test file with failing tests that define the expected behavior.
2. Show me the tests. Confirm they fail.
3. Write the implementation.
4. Run tests. Confirm they pass.
5. Refactor if needed.
6. Never skip to step 3.

## SOLID + Interface Segregation (ISPF)

### Single Responsibility
- One module = one job. Provider adapters fetch. Storage persists. Commands dispatch. Components render.
- If a module has two reasons to change, split it.

### Open/Closed
- Add providers by implementing traits. Never modify existing providers to add a new one.
- The polling engine, storage layer, and UI accept any conforming implementation.

### Liskov Substitution
- Any `impl ProviderAdapter` is drop-in interchangeable. Mocks behave like real adapters.
- If a test passes with MockProvider, it passes with RealProvider (modulo network).

### Interface Segregation (Enforce Strictly)
- **Small, focused traits.** No trait with more than 5 methods. Split by capability:
  ```rust
  trait UsageFetcher    { fn fetch_usage(&self) -> Result<UsageData>; }
  trait BalanceChecker  { fn check_balance(&self) -> Result<Balance>; }
  trait TransactionLog  { fn recent_transactions(&self, limit: usize) -> Result<Vec<Transaction>>; }
  ```
- A provider implements ONLY the traits it supports. No stub methods. No `unimplemented!()`.
- Svelte component props: pass exactly what's needed. A gauge takes `value: number` and `max: number`, not `provider: Provider`.
- Tauri commands: one command, one action. No `execute_action(action: String, payload: Value)`.

### Dependency Inversion
- Core logic depends on traits, never on concrete types.
- `PollingEngine` takes `Vec<Box<dyn UsageFetcher>>`, not `Vec<AnthropicAdapter>`.
- Frontend accesses backend through typed API wrappers in `src/lib/api/`, not raw `invoke()`.
- Storage through `Repository` trait, not direct `rusqlite::Connection`.

## Rust Standards

- Edition 2021, stable toolchain.
- `thiserror` for library errors, `anyhow` only at the application boundary.
- No `unwrap()` in production. Use `?` everywhere. `expect()` only for provably impossible states with a message explaining why.
- No `unsafe` unless documented with `// SAFETY:` explaining the invariant.
- All HTTP requests: 30s timeout, retry with backoff for transient failures.
- Provider errors are typed enums: `RateLimit`, `AuthFailed`, `NetworkError`, `ParseError`.
- One provider failing does not affect others. The polling engine continues.
- Secrets accessed exclusively via `keyring` crate. Never in logs, never in error messages, never serialized.
- `cargo clippy -- -D warnings` must pass.
- `cargo fmt` before every commit.

## Svelte 5 + TypeScript Standards

- Svelte 5 runes: `$state`, `$derived`, `$effect`. No legacy store syntax.
- TypeScript `strict: true`. Zero `any` types. Define interfaces for all data shapes.
- Components under 150 lines. Extract when growing.
- Presentation components: pure rendering, props in, events out. No side effects.
- Container components: manage data, invoke API layer, pass data down.
- Scoped `<style>` blocks. CSS custom properties for theming.
- ARIA labels on interactive elements. Keyboard-navigable HUD.

## Architecture

```
src-tauri/src/
  providers/         # One module per AI tool
    anthropic/       # Claude Code + Claude
    cursor/
    openai/          # Codex + Copilot
    windsurf/
    kilo/
    mod.rs           # Trait definitions
  polling/           # Periodic fetch engine
  storage/           # SQLite behind Repository trait
  keychain/          # OS keychain behind trait
  commands/          # Tauri commands — thin dispatch layer
  models/            # Usage, Rate, Balance, Transaction types
  errors/            # Typed error hierarchy

src/lib/
  api/               # Typed Tauri invoke wrappers
  components/
    tray/            # Menubar numbers
    hover/           # Provider breakdown popover
    hud/             # Full translucent overlay
    gauges/          # Tachometers, sparklines, charts
    shared/          # Reusable primitives
  stores/            # Svelte 5 rune-based state
  types/             # TS interfaces (mirror Rust models)

tests/
  e2e/               # End-to-end
  fixtures/          # Mock API responses
```

## Security (Non-Negotiable)

- API keys: OS keychain only. Never plaintext. Never logged. Never in error messages.
- Pre-commit hook scans for secrets. Do not bypass it.
- All provider communication: HTTPS only.
- No telemetry, no analytics, no phoning home.
- User data stays local (SQLite). No cloud sync in MVP.

## Definition of Done

Before any feature is complete:
- [ ] Tests written first (red)
- [ ] Tests pass (green)
- [ ] Code refactored, tests still green
- [ ] `cargo clippy -- -D warnings` clean
- [ ] `cargo fmt` applied
- [ ] TypeScript strict — no errors
- [ ] Prettier formatted
- [ ] No secrets in code, comments, or logs
- [ ] Errors handled with typed results — no panics, no ignoring
- [ ] Interface segregation verified — traits and props are minimal
- [ ] Doc comments on public Rust APIs

## Do NOT

- Write code before tests.
- Use `any` in TypeScript.
- Store secrets outside OS keychain.
- Log sensitive data.
- Create god-traits or god-components.
- Pass full objects when a subset suffices.
- Ignore errors with `let _ =` or `.ok()`.
- Use string error messages instead of typed errors.
- Add dependencies without clear justification.
- Write order-dependent or flaky tests.
- Commit without running the test suite.
- Skip the TDD cycle for "simple" changes — there are no simple changes.
