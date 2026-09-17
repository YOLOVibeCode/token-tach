# Provider API Research — Token Tach

Research completed 2026-05-15. This determines what we can actually build.

---

## Feasibility Matrix

| Provider | API Exists | Auth Needed | Granularity | Individual Users | Feasibility |
|---|---|---|---|---|---|
| **Anthropic** | ✅ Admin API | Admin key (`sk-ant-admin-...`) | 1min / 1hr / 1day | ❌ Org only | ★★★★☆ |
| **OpenAI** | ✅ Admin API | Admin key (`sk-admin-...`) | Daily (costs), per-request (usage in responses) | ❌ Org admin only | ★★★★☆ |
| **Cursor** | ✅ Enterprise API | Admin API key | Daily + per-event | ❌ Enterprise only | ★★★☆☆ |
| **GitHub Copilot** | ✅ REST API | PAT / OAuth | Daily aggregate | ❌ Org only | ★★★★☆ |
| **Windsurf** | ❌ No public API | Service key (limited) | Not documented | ❌ Dashboard only | ★★☆☆☆ |
| **Kilo Code** | ⚠️ Dashboard only | Provider tokens | Per-request (µ$) | ✅ Yes | ★★★☆☆ |

---

## The Hard Truth

**Every provider requires admin/org-level API keys.** Individual developers on personal plans cannot programmatically access their own usage data through official APIs. This is the single biggest limitation.

---

## Anthropic (Claude Code + claude.ai)

### Endpoints
- `GET /v1/organizations/usage_report/messages` — token usage (1min/1hr/1day buckets)
- `GET /v1/organizations/cost_report` — cost in USD (daily only)
- `GET /v1/organizations/usage_report/claude_code` — Claude Code productivity metrics

### Auth
- **Admin API key required** (prefix: `sk-ant-admin-...`)
- Only org admins can create these in Claude Console → Settings → Admin Keys
- NOT available for individual/free accounts
- NOT available on Claude Platform on AWS

### Response Data
- Usage: input/output/cached tokens, model, service tier, workspace, per-minute granularity
- Cost: daily USD amounts by model, token type, service tier
- Claude Code: sessions, lines of code, commits, PRs, tool actions, estimated cost

### Limitations
- 5-minute data freshness delay
- No per-request granularity (1-minute minimum)
- Cost API is daily only (usage API has minute-level)
- Priority Tier costs excluded from cost API

### Local Data
- `~/.claude/stats-cache.json` — daily message/session/tool counts (no costs or tokens)
- `~/.claude/history.jsonl` — conversation history (no cost data)

---

## OpenAI (Codex + GPT models)

### Endpoints
- `POST /v1/organization/usage/costs` — daily cost breakdown with line items
- `GET /v1/organization/usage/completions` — token usage per capability
- `GET /v1/dashboard/billing/credit_grants` — remaining credit balance
- Per-request: every API response includes `usage` object with token counts

### Auth
- **Organization Admin API key** (prefix: `sk-admin-...`)
- Only org owners/admins can access billing endpoints
- Admin keys CANNOT be used for chat/completions (separate from regular keys)
- Regular keys DO get per-request token counts in response objects

### Response Data
- Costs: amounts in cents, line items by model/project/API key, daily buckets
- Usage: input/output/cached/reasoning tokens, per-request in response objects
- Credits: remaining balance, grant details

### Limitations
- Cost amounts are in cents (divide by 100)
- No sub-daily cost granularity
- Admin key required for full billing data

### Local Data (Codex CLI)
- `~/.codex/history.jsonl` — session transcripts
- `~/.codex/config.toml` — settings
- No token/cost data stored locally

---

## Cursor

### Endpoints (Enterprise Only)
- `POST /teams/daily-usage-data` — daily aggregated usage
- `POST /teams/filtered-usage-events` — per-request usage events with cost
- `GET /teams/user-spend-limit` — spending limits
- Analytics API: agent edits, tab completions, DAU, model usage, leaderboard
- Base URL: `https://api.cursor.com`

### Auth
- Admin API key from `cursor.com/dashboard`
- **Enterprise plans only** — not available on Hobby/Pro/Teams

### Local Data
- `~/.cursor/ai-tracking/ai-code-tracking.db` — SQLite with code attribution (not costs)
- `~/Library/Application Support/Cursor/` — VSCode databases, logs, workspace data
- Network logs show `api2.cursor.sh`, `api3.cursor.sh` endpoints

### Community Tools
- [cursor-usage MCP plugin](https://github.com/ofershap/cursor-usage) — wraps Enterprise API
- [Cursor Usage Widget](https://cursorusage.com/) — macOS menu bar app (similar concept to Token Tach!)
- [cursor-usage-tracker](https://github.com/ofershap/cursor-usage-tracker) — monitoring + alerting

### Billing Model
- Subscription ($20-200/mo individual, $40/user teams)
- Plus usage-based overages at model API rates (~$1.25/M input, ~$6/M output)

---

## GitHub Copilot

### Endpoints
- `GET /orgs/{org}/copilot/usage_metrics/day/{date}` — daily per-user usage
- `GET /orgs/{org}/copilot/metrics/reports/user-teams-1-day` — team aggregates (new May 2026)
- 28-day aggregation window

### Auth
- Personal Access Token or OAuth with `manage_billing:copilot` scope
- Organization owners, billing managers, or enterprise admins only
- **No individual access API**

### Billing Model
- Moving to AI Credits (June 1, 2026): $0.01 per credit, based on token consumption

### Local Data
- OTel spans in local SQLite if `github.copilot.chat.otel.dbSpanExporter.enabled=true`

---

## Windsurf (Codeium)

### Endpoints
- **No public usage/billing API**
- Service key API for code completion only
- Web dashboard at windsurf.com/subscription shows credits

### Local Data
- `~/.codeium/windsurf/cascade/` — conversation history
- `~/.codeium/windsurf/implicit/` — protobuf cache
- No usage metrics stored locally

### Billing Model
- Credits system (1 credit = $0.04), 500-1000+ credits/month on paid plans

### Verdict
**Not viable without Windsurf publishing an API.** Deprioritize.

---

## Kilo Code

### Endpoints
- **Dashboard only** — no REST API documented
- Dashboard tracks per-request usage at microdollar precision

### Local Data
- VS Code SecretStorage for API keys
- Config files, conversation history
- No usage metrics locally

### Billing Model
- Zero markup pass-through to underlying providers
- Kilo is a meta-provider (proxy for OpenAI, Anthropic, etc.)

### Verdict
Track underlying providers directly. Kilo-specific tracking needs their API.

---

## Recommended Build Strategy

### Tier 1 — Build Now (MVP)
1. **Anthropic** — Admin API with usage + cost + Claude Code analytics endpoints
2. **OpenAI** — Admin API with costs + per-capability usage endpoints

### Tier 2 — Build Next
3. **Cursor** — Enterprise API for teams; explore local SQLite DB as fallback
4. **GitHub Copilot** — REST API with PAT auth for organizations

### Tier 3 — Wait for APIs
5. **Kilo Code** — Track underlying providers instead
6. **Windsurf** — No path forward without their API

### Alternative Approach: Local Log Parsing
For individual (non-org) users, consider:
- Parse `~/.claude/stats-cache.json` for Claude Code activity
- Parse Codex `~/.codex/history.jsonl` for session data
- Query Cursor's `ai-code-tracking.db` for code attribution
- Read Copilot OTel SQLite if telemetry enabled

This gives **activity metrics** (sessions, messages, edits) but **not costs** — would need user-configured cost-per-token rates to estimate spend.
