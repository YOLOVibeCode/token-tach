# Token Tach — Requirements Document

## Overview

Token Tach is a lightweight desktop utility that acts as a tachometer for AI coding tool spending. It lives in the system tray / menubar and provides at-a-glance visibility into token usage, credit balances, and burn rates across all major AI coding assistants — the same way your OS shows CPU, battery, and memory.

## Problem

Developers using AI coding tools have no unified way to see what they're spending in real time. Credits drain silently across multiple platforms. By the time you check a dashboard, you've already blown through your budget. There's no "gas gauge" for AI spend.

## Solution

Three-layer interaction model:

1. **Tray / Dock — Two Numbers** — always visible: burn rate + monthly cost
2. **Hover — Provider Breakdown** — each tool's burn rate + cost at a glance
3. **Click — Full HUD** — translucent overlay with detailed transactions, graphs, gauges

---

## Supported Providers

| Provider | API/Method |
|---|---|
| Claude Code | Anthropic API usage endpoints |
| Claude (claude.ai) | Anthropic billing/usage API |
| Cursor | Cursor account/billing API |
| Windsurf (Codeium) | Codeium billing/usage API |
| GitHub Copilot | GitHub Copilot usage API |
| OpenAI Codex | OpenAI usage/billing API |
| Kilo Code | Kilo usage/billing API |

Additional providers should be pluggable via a provider adapter interface.

---

## Core Features

### Layer 1: System Tray + Dock — The Two Numbers

Always visible. Two numbers that tell you everything at a glance:

```
  [$3.41/hr]  [$247.80]
   burn rate   monthly cost
```

- **Number 1 — Burn Rate:** current aggregate spend velocity across all providers ($/hr)
- **Number 2 — Monthly Cost:** total spend for the current billing period (configurable in settings: calendar month, rolling 30 days, or custom cycle start date)
- Displayed in the **macOS menubar** (system tray) and/or **Dock badge**
- Color-coded by budget status:
  - **Green** — under 50% of monthly budget
  - **Yellow** — 50-80% of monthly budget
  - **Red** — over 80% of monthly budget
  - **Pulsing** — tokens actively flowing right now
- Settings control: which numbers to show, billing period, budget thresholds

### Layer 2: Hover — Provider Breakdown

Hover over the tray icon or Dock icon to see a compact tooltip/popover showing **every connected tool with the same two numbers each:**

```
  ┌─────────────────────────────────┐
  │  Token Tach        $3.41/hr     │
  │  May 2026          $247.80      │
  │─────────────────────────────────│
  │  ◉ Cursor          $1.85/hr  $142.30 │
  │  ◉ Claude Code     $1.20/hr   $78.50 │
  │  ◉ Codex           $0.36/hr   $27.00 │
  │  ◎ Copilot         idle        $0.00 │
  │  ◎ Windsurf        idle        $0.00 │
  └─────────────────────────────────┘
```

- **Per provider:** burn rate + monthly cost — same two numbers
- Status dot: filled = active, hollow = idle, red = error
- Sorted by current burn rate (hottest on top)
- Compact — no clicking required, just hover and glance
- Disappears when mouse moves away

### Layer 3: Click — Translucent HUD (The Full Display)

Click the tray icon, Dock icon, or press a global hotkey to open the **full heads-up display** — a translucent, glassmorphic overlay panel. This is where you see the detailed transaction-level view of everything.

#### Header

- Aggregate burn rate (large) + monthly cost (large)
- Billing period label (e.g., "May 2026") — configurable in settings
- Budget remaining — circular progress ring
- Session timer — current coding session duration

#### Provider Detail Cards

Each provider gets an expanded card:
- Provider logo + name + connection status
- **Burn rate** + **monthly cost** (the two numbers, prominent)
- Model currently in use (e.g., "claude-opus-4-6", "gpt-4o")
- Tokens used: input / output breakdown
- Remaining credits / balance (if applicable)
- Mini sparkline — burn rate over the last 60 minutes
- Trend arrow (burn rate going up / down / stable)

#### Transaction Log

The detailed view — every API interaction:
- Scrollable list of recent transactions
- Each row: timestamp, provider, model, input tokens, output tokens, cost
- Color-coded by provider
- Filterable by provider, date range, model
- Sortable by cost, time, token count
- Search within transactions
- Running cost total at the bottom

#### Tachometer Gauges

- Arc/dial gauges per provider — needle shows current burn rate
- Redline zone = budget danger threshold
- Center master gauge = aggregate burn rate
- Satisfying needle sweep animations

#### Spend Over Time Graph

- Area chart: spend over the billing period
- Stacked by provider, color-coded
- Budget limit line overlaid
- Hover for exact values at any point in time
- Toggle: daily / weekly / full month views

#### Cost Projections

- "At this rate: $X by end of month"
- Dashed projection line on the spend graph
- Warning if projection exceeds budget

#### Token Efficiency

- Input vs output token ratio per provider
- Average cost per interaction
- Most expensive conversation/session today
- Cache hit rate (where supported)

#### Display Behavior

- **Translucent / glassmorphic** — frosted glass with vibrancy, content behind is blurred
- **Always on top** but dismissable (click outside, Esc, or hotkey toggle)
- **Draggable** — position anywhere on screen
- **Resizable** — compact, standard, and expanded modes
- **Dark mode native** — matches system appearance
- **Smooth animations** — gauges sweep, sparklines draw, numbers count up
- **Pinnable** — pin to screen, fades to ~20% opacity until hovered

### 3. Registration and API Access

Dead simple setup:

- First launch opens a setup wizard
- For each provider: one field for the API key or OAuth login
- "Test Connection" button per provider — instant green checkmark or error
- Skip any provider — add later from settings
- Store credentials securely in OS keychain (macOS Keychain, Windows Credential Manager, libsecret on Linux)
- Total setup time target: **under 2 minutes** for all providers

### 4. Alerts and Budgets

- User-configurable spending limits (daily, weekly, monthly)
- Desktop notifications when thresholds are hit (50%, 75%, 90%, 100%)
- Optional hard-stop alert: "You've spent $X today — pause and review?"
- Per-provider and aggregate budget tracking

### 5. Usage History

- Local SQLite database for usage history
- Simple charts: daily/weekly/monthly spend by provider
- Export to CSV
- No cloud account required — all data stays local

---

## Non-Functional Requirements

### Performance
- Polling interval: configurable, default 30 seconds
- HUD render: < 100ms on hover
- Memory footprint: < 50MB resident
- CPU: negligible at idle

### Security
- API keys stored in OS keychain only — never plaintext on disk
- No telemetry or data sent to Token Tach servers
- All provider communication over HTTPS
- Open source — auditable

### Privacy
- Zero data collection
- No account required to use Token Tach
- All usage data stored locally

---

## Tech Stack (Recommended)

| Layer | Technology |
|---|---|
| Desktop shell | Tauri v2 (Rust + webview) |
| UI framework | Svelte 5 (reactive, lightweight, fast) |
| Translucent window | Tauri window vibrancy plugin (NSVisualEffectView on macOS) |
| System tray | Tauri tray API with dynamic icon generation |
| Dock badges | macOS NSDockTile API via Tauri plugin |
| Gauges / dials | Custom SVG + CSS animations (or D3.js) |
| Sparklines | uPlot (tiny, fast) or custom Canvas |
| Spend charts | D3.js or Chart.js with custom theme |
| Local storage | SQLite via rusqlite |
| Credential storage | OS keychain via keyring crate |
| Provider adapters | Rust async traits + reqwest |
| Real-time updates | Tauri event system (Rust -> frontend push) |
| Packaging | Tauri bundler (dmg, msi, AppImage) |

Tauri is preferred over Electron for the small footprint requirement. The translucent overlay requires native vibrancy APIs — Tauri supports this via `window-vibrancy` plugin on macOS (NSVisualEffectView), Windows (Acrylic/Mica), and Linux (limited).

---

## User Flow

```
Install -> Launch -> Setup Wizard
  -> Enter API keys per provider
  -> Test connections -> green checkmarks
  -> Set billing period + monthly budget
  -> Done

Daily use:
  -> Glance at menubar: "$3.41/hr  $247.80" (green)
  -> Hover: see Cursor is $1.85/hr, Claude Code $1.20/hr
  -> Keep coding
  -> Numbers turn yellow — hover to check who's burning
  -> Click: full HUD opens, translucent overlay
  -> Scroll transaction log — see a heavy Cursor session
  -> Check gauges, review the spend graph
  -> Dismiss with Esc, keep coding
  -> Get notified at 80% of monthly budget
```

---

## MVP Scope (v0.1)

1. macOS menubar tray icon with color-coded burn rate number
2. Mac Dock badge with live spend summary
3. Translucent overlay display with:
   - Provider cards (status, spend, burn rate, sparkline)
   - Tachometer-style gauges
   - Today's spend graph
   - Budget progress ring
4. Support for 3 providers: Claude (Anthropic), OpenAI (Codex/Copilot), Cursor
5. API key setup wizard with keychain storage
6. Local SQLite usage tracking
7. Desktop notifications at budget thresholds
8. Global hotkey to toggle overlay

## v0.2

- Windows and Linux support
- Windsurf, Kilo Code, GitHub Copilot providers
- Real-time activity feed (API call ticker)
- Cost projection engine
- Token efficiency panel
- Resizable overlay (compact / standard / expanded)
- CSV export

## v0.3

- Custom provider plugin system
- Team/org dashboard (optional cloud sync)
- CLI companion (`token-tach status`)
- Pinned low-opacity idle mode
- Widget kit for macOS desktop widgets

---

## Success Metrics

- Setup completed in under 2 minutes
- Burn rate displayed within 30 seconds of first launch
- Memory usage under 50MB
- User can answer "how much am I spending on AI tools today?" in one glance
