# Token Tach

## What It Is
AI coding tool spend monitor — a macOS menubar/dock utility that shows real-time burn rate and monthly cost across all your AI coding tools.

## Three-Layer UI
1. **Tray/Dock** — two numbers always visible: burn rate ($/hr) + monthly cost
2. **Hover** — per-provider breakdown (each tool's burn rate + cost)
3. **Click** — translucent glassmorphic HUD with transaction logs, tachometer gauges, spend graphs, projections

## Supported Providers
| Provider | Status |
|---|---|
| Claude Code / Claude | MVP |
| Cursor | MVP |
| OpenAI Codex / Copilot | MVP |
| Windsurf (Codeium) | v0.2 |
| GitHub Copilot | v0.2 |
| Kilo Code | v0.2 |

## Tech Stack
| Layer | Technology |
|---|---|
| Desktop shell | Tauri v2 (Rust) |
| Frontend | Svelte 5 (TypeScript strict) |
| Storage | SQLite (rusqlite, WAL mode) |
| Credentials | OS Keychain (keyring crate) |
| HTTP | reqwest (rustls-tls) |
| Tests | vitest (frontend), cargo test (backend) |
| CI/CD | GitHub Actions + release-please |

## Repository
- **GitHub:** https://github.com/YOLOVibeCode/token-tach
- **License:** MIT (YOLOVibeCode)

## Related Docs
- [[Token Tach - Apple Developer Setup]]
- [[Token Tach - Release Pipeline]]
- [[Token Tach - Architecture]]
- [[Token Tach - Implementation Progress]]
