# Implementation Progress — Token Tach

## Architecture Pivot (2026-05-15)

API research revealed all providers lock billing APIs behind org admin keys. Individual developers can't access their own spend data via API.

**Solution: Dual-Mode Architecture**
- **API Mode** (exact) — for org admins with admin API keys
- **Local Mode** (estimated) — parses local activity logs, user-configured rates. Works for everyone.
- Local Mode is the **default** — zero config needed to start

---

## Phase Status

| Phase | Status | Tests | Description |
|---|---|---|---|
| **Phase 0** | Done | 1 TS | Scaffold: Tauri v2 + Svelte 5, CI/CD, release-please |
| **Phase 1** | Done | 61 Rust + 14 TS | Core models, error types, 5 ISP provider traits |
| **Phase 2** | Done | 28 Rust | Keychain + SQLite storage (4 ISP traits) |
| **Phase 3a** | Pending | — | Anthropic adapter (API + local mode) |
| **Phase 3b** | Pending | — | OpenAI adapter (API + local mode) |
| **Phase 3c** | Pending | — | Cursor adapter (API + local mode) |
| **Phase 4** | Pending | — | Dual-mode polling, burn rate, cost estimator, commands |
| **Phase 5** | Pending | — | Frontend API + stores + settings |
| **Phase 8g** | Pending | — | Setup wizard (auto-detect + optional API keys) |
| **Phase 6** | Pending | — | System tray (two numbers) |
| **Phase 7** | Pending | — | Hover popover (API/EST badges per provider) |
| **Phase 8a** | Pending | — | HUD window + layout |
| **Phase 8b-f** | Pending | — | HUD features (cards, log, gauges, graphs, efficiency) |
| **Phase 8h** | Pending | — | Budget alerts |

## Running Totals
- **Rust tests:** 89
- **TypeScript tests:** 14
- **Total tests:** 103

## Local Data Sources (per provider)

| Provider | Local Path | Data Available |
|---|---|---|
| Claude Code | `~/.claude/stats-cache.json` | Daily message/session/tool counts |
| Claude Code | `~/.claude/history.jsonl` | Conversation history with timestamps |
| Codex | `~/.codex/history.jsonl` | Session transcripts |
| Cursor | `~/.cursor/ai-tracking/ai-code-tracking.db` | SQLite: code attribution, conversations, models |
| Copilot | VS Code OTel SQLite (if enabled) | Spans with token data |

## API Endpoints (per provider)

| Provider | Key Endpoint | Auth Required |
|---|---|---|
| Anthropic | `/v1/organizations/usage_report/messages` | Admin key `sk-ant-admin-...` |
| OpenAI | `/v1/organization/usage/costs` | Admin key `sk-admin-...` |
| Cursor | `/teams/daily-usage-data` | Enterprise admin key |
| Copilot | `/orgs/{org}/copilot/usage_metrics/day/{date}` | PAT with billing scope |

## MVP Definition (v0.1.0)

Shippable when:
- [x] CI/CD + release pipeline
- [x] Core models + storage
- [x] Apple signing configured
- [ ] Provider adapters (Anthropic + OpenAI + Cursor, both modes)
- [ ] Polling engine + burn rate calculator
- [ ] Setup wizard with auto-detection
- [ ] System tray (two numbers)
- [ ] Hover popover (provider breakdown with API/EST badges)
- [ ] Budget alerts
- [ ] App icon
- [ ] README

## Related Docs
- [[Token Tach - Project Overview]]
- [[Token Tach - Provider API Research]]
- [[Token Tach - Architecture]]
- [[Token Tach - Apple Developer Setup]]
- [[Token Tach - Release Pipeline]]
