# Architecture — Token Tach

## Principles
- **TDD:** Red-Green-Refactor on every change. Tests first, always.
- **SOLID/ISPF:** Small focused traits. Providers implement only what they support.
- **Dependency Inversion:** Core logic depends on traits, never concrete types.

## Rust Backend (`src-tauri/src/`)

```
models/           Shared types (UsageData, BurnRate, Balance, Transaction,
                  ProviderId, ProviderStatus, Budget, BillingPeriod)

errors/           Typed error hierarchy
                  ProviderError (RateLimit, AuthFailed, NetworkError, ParseError, NotSupported)
                  StorageError (ConnectionFailed, QueryFailed, MigrationFailed)
                  KeychainError (NotFound, AccessDenied, StoreError)
                  AppError (wraps all via From)

providers/        5 ISP-compliant traits:
  mod.rs            UsageFetcher, BalanceChecker, TransactionFetcher,
                    ProviderInfo, ConnectionTester
  anthropic/        Claude Code + claude.ai adapter
  cursor/           Cursor adapter
  openai/           Codex + Copilot adapter

keychain/         SecretStore trait + OsSecretStore (keyring crate)

storage/          4 ISP-split traits:
  mod.rs            TransactionWriter, TransactionReader, UsageSummary, BudgetStore
  sqlite.rs         SqliteRepository (WAL mode, in-memory for tests)

polling/          PollingEngine (periodic multi-provider fetch)
                  BurnRateCalculator (sliding window)
                  CostProjector (linear projection)

commands/         Tauri commands — thin dispatch layer
  usage.rs          burn rate, monthly cost, force refresh
  providers.rs      list, test connection, set/remove API key
  transactions.rs   recent, since
  budget.rs         set, get, status

tray/             System tray (two numbers: burn rate + monthly cost)

alerts/           Budget threshold notifications
```

## Svelte 5 Frontend (`src/lib/`)

```
api/              Typed Tauri invoke wrappers (no raw invoke in components)
  usage.ts          getAggregateBurnRate, getProviderBurnRates, etc.
  providers.ts      listProviders, testConnection, setApiKey, etc.
  transactions.ts   getRecentTransactions, getTransactionsSince
  budget.ts         setBudget, getBudget, getBudgetStatus
  events.ts         Tauri event listeners

stores/           Svelte 5 runes ($state, $derived, $effect)
  usage.svelte.ts
  providers.svelte.ts
  transactions.svelte.ts
  budget.svelte.ts

components/
  tray/             Menubar display
  hover/            HoverPopover, ProviderRow, StatusDot
  hud/              Full translucent overlay
    HudOverlay, HudHeader, ProviderCard, TransactionLog,
    TransactionRow, SpendGraph, CostProjection, TokenEfficiency
  gauges/           Tachometer, TachometerGrid, Sparkline, TrendArrow, ProgressRing
  shared/           FormatCurrency, FormatRate
  setup/            SetupWizard, ProviderSetupCard, BudgetSetup

types/            TypeScript interfaces mirroring Rust models
```

## Key Design Rules
- **One provider failing does NOT block others** — polling engine isolates errors
- **API keys in OS keychain only** — never in files, logs, or error messages
- **Commands are thin dispatch** — no business logic, delegate to traits
- **Frontend never calls raw `invoke()`** — everything through typed API wrappers
- **Props follow ISP** — pass only what a component needs, not full objects
